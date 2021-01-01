(ns advent.day20
  (:require [clojure.string :as str]))

(defn split [re s]
  (str/split s re))

(def ex1 "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...")

(def block
  "#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...")

(defn parse-data [s]
  (->> s
       str/trim
       (split #"\n\n")
       (map (fn [s] [(Integer/parseInt (re-find #"\d+" s))
                     (->> (.substring s 11)
                          str/split-lines)]))
       (into {})))

(defn rotate-tile [tile]
  (->> tile
       reverse
       (apply map str)))

(defn flip-tile [tile]
  (->> tile
       reverse))

(defn comb-tile [tile]
  (let [rt (->> tile
                (iterate rotate-tile)
                (take 4))
        vt (->> tile
                flip-tile
                (iterate rotate-tile)
                (take 4))]
    (concat rt vt)))

(def data (->> ex1
               parse-data))

(defn edges [tile]
  (let [left (->> tile
                  (map first)
                  (apply str))
        top (first tile)
        right (->> tile
                   (map last)
                   (apply str))
        bottom (last tile)]
    [left top right bottom]))

(defn matching-edge? [t1 t2]
  (let [[t1l t1t t1r t1b] (edges t1)
        [t2l t2t t2r t2b] (edges t2)]
    (or (= t1l t2r)
        (= t1t t2b)
        (= t1r t2l)
        (= t1b t2t))))

(defn matching-tile? [t1 t2]
  (->> t2
       comb-tile
       (some #(matching-edge? t1 %))))

(def input (->> "resources/day20-input.txt"
                slurp
                parse-data))

(defn part1 [data]
  (let [m-matching-tile? (memoize
                          (fn [ids]
                            (->> ids
                                 (map #(get data %))
                                 (apply matching-tile?))))]
    (->> data
         (filter (fn [[key _]]
                   (->> data
                        (filter (fn [[k _]]
                                  (and (not= k key)
                                       (m-matching-tile? #{k key}))))
                        count
                        (= 2))))
         (map first)
         (apply *))))

;; 15670959891893

(defn matching-edge [t1 t2]
  (let [[t1l t1t t1r t1b] (edges t1)
        [t2l t2t t2r t2b] (edges t2)]
    (cond
        (= t1l t2r) :left
        (= t1t t2b) :top
        (= t1r t2l) :right
        (= t1b t2t) :bottom
        :else nil)))

(defn matching-tile [t1 t2]
  (->> t2
       comb-tile
       (map (fn [t] [(matching-edge t1 t) t]))
       (filter first)
       first))

(let [data (seq data)
      [fid ftile] (first data)
      r (rest data)]
  (loop [data r
         grid [[0 0 fid ftile]]]
    (map (fn [[r c id tile]]
           (->> data
                (map (fn [[cid t]] [cid (matching-tile tile t)]))
                (filter (comp not nil? second))))
         grid)))

(defn queue
  ([] (clojure.lang.PersistentQueue/EMPTY))
  ([coll]
   (reduce conj clojure.lang.PersistentQueue/EMPTY coll)))

(defn queue-rotate [q]
  (let [p (peek q)]
    (conj (pop q) p)))

(queue-rotate (queue [1 2 3 4]))

(defn match-tiles [tiles [pid t]]
  (let [matched (->> tiles
                     (map (fn [[cid ct]] (into [cid pid] (matching-tile t ct))))
                     (filter #(> (count %) 2)))
        remaining-tiles (->> matched
                             (map first)
                             (apply dissoc tiles))]
    [remaining-tiles matched]))

(defn concat-queue [v q]
  (reduce conj (queue v) q))

(concat-queue [1 2 3] (queue [4 5 6]))

(defn build-graph [data]
 (loop [[id t :as p] (first data)
        tiles (dissoc data id)
        matched (queue [[id 0 nil t]])]
   (if (empty? tiles)
     matched
     (let [[remaining-tiles new-matched] (match-tiles tiles p)
           matched (concat-queue new-matched matched)
           [id _ _ t :as f] (peek matched)]
       (recur [id t] remaining-tiles (conj (pop matched) f))))))

(->> data
     build-graph
     (take 2)
     (map (fn [[key & rest]] [key (vec rest)]) )
     (into {}))

(let [[a & b] [1 2 3 4]]
  [a (vec b)])

(defn find-parent-graph [graph pid]
  (filter #(= (second %) pid) graph))

(defn remove-item-graph [graph id]
  (remove #(= (first %) id) graph))

(defn remove-items-graph [graph ids]
  (reduce (fn [acc id] (remove-item-graph acc id)) graph ids))

(defn mark-coords [graph]
  (let [origin (->> graph
                    (filter (comp zero? second))
                    first)]
    (loop [graph (remove-item-graph graph (first origin))
           q (queue [(conj origin [0 0])])]
      (if (empty? graph)
        q
        (let [[cid _ _ _ coord] (peek q)
              neighbors (find-parent-graph graph cid)
              marked (map (fn [[_ _ dir _ :as d]]
                            (conj d (map + coord (case dir
                                                   :left [-1 0]
                                                   :right [1 0]
                                                   :top [0 -1]
                                                   :bottom [0 1]))))
                          neighbors)]
          (recur (remove-items-graph graph (map first marked))
                 (concat-queue marked (queue-rotate q))))))))

(defn place-grid [grid]
  (let [[[nx _] [ny _] :as minmax]
        (->> grid
             (map last)
             (reduce (fn [[[minx maxx] [miny maxy]] [x y]]
                       [[(min minx x) (max maxx x)] [(min miny y) (max maxy y)]])
                     [[(Integer/MAX_VALUE) (Integer/MIN_VALUE)]
                      [(Integer/MAX_VALUE) (Integer/MIN_VALUE)]]))
        [col row] (->> minmax
                       (map (fn [[mn mx]] (- mx mn -1)))
                       vec)
        place (vec (repeat row (vec (repeat col nil))))]
    (reduce (fn [place tile]
              (let [coord (map - (last tile) [nx ny])]
                (assoc-in place (reverse coord) tile)))
            place grid)))

(defn part1-1 [data]
  (let [canvas (->> data
                    build-graph
                    mark-coords
                    place-grid)]
    (->> [(->> canvas
               first
               first)
          (->> canvas
               first
               last)
          (->> canvas
               last
               first)
          (->> canvas
               last
               last)]
         (map first)
         (apply *))))


(def data-grid (->> data
                    build-graph
                    mark-coords
                    place-grid))

(defn remove-border-tile [tile]
  (->> tile
       (map #(.substring % 1 (- (.length %) 1)))
       (drop 1)
       drop-last))

(def monster ["                  # "
              "#    ##    ##    ###"
              " #  #  #  #  #  #   "])

(defn draw-sea [data]
  (let [tiles (->> data
                   (map (fn [row] (map #(nth % 3) row))))
        row (count tiles)
        col (count (first tiles))
        borderless (map (fn [row] (map remove-border-tile row)) tiles)
        sample (first (first borderless))
        height (count sample)
        width (.length (first sample))
        img (->> borderless
                 (map (fn [row] (apply map str row)))
                 flatten
                 vec)]
    [img [(* row height) (* col width)]]))

(defn partition-image [img w h]
  (->> img
       (map (fn [line]
              (map #(apply str %)
                   (partition w 1 line))))
       (apply map vector)
       (map #(partition h 1 %))
       (reduce into [])))

(def m ["123456789"
        "abcdefghi"
        "jklmnopqr"
        "stuvwxyz!"
        "@#$%^&*()"
        ";':?><,./"])
(partition-image m 7 5)

(defn match-monster-dot? [[m t]]
  (or (not= \# m) (= \# t)))

(defn match-sea-dot? [[m t]]
  (and (not= \# m) (= \# t)))

(defn rough-count [monster t2]
  (let [dots (->> (map vector monster t2)
                  (map #(apply map vector %))
                  (reduce into []))
        match? (every? match-monster-dot? dots)]
    match?))

(def sea ["####              # "
          "#    ##    ##    ###"
          " #  #  #  #  #  #   "])

(rough-count monster sea)

(defn dot-count [tile]
  (->> tile
       (map (fn [line] (->> line
                            (filter #(= \# %))
                            count)))
       (apply +)))

(dot-count sea)

(defn rough [data]
  (let [[tile _] (draw-sea data)
        mrow 3
        mcol 20
        mcount (dot-count monster)
        tiles (->> tile
                   comb-tile
                   (map (fn [tile] [tile (partition-image tile mcol mrow)])))]
    (->> tiles
         (map (fn [[t parts]] [t (->> parts
                                      (filter #(rough-count monster %))
                                      count)]))
         (filter (fn [[_ c]] (> c 0)))
         (map (fn [[t c]] (- (dot-count t) (* mcount c))))
         (apply +))))

(def data-input (->> input
                     build-graph
                     mark-coords
                     place-grid))

(rough data-input)
