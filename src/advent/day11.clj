(ns advent.day11
  (:require [clojure.java.io :as io]
            [clojure.string :as str]))

(def F "resources/day11-input.txt")

(defn create-layout [seats rows cols]
  {:seats seats :rows rows :cols cols})

(defn get-cell [layout [row col]]
  (let [pos (+ (* row (:cols layout)) col)]
    (nth (:seats layout) pos)))

(defn adjacency [layout [row col :as coord]]
  (let [cols (:cols layout)
        rows (:rows layout)
        left (max (- col 1) 0)
        right (min (+ col 1) (- cols 1))
        top (max (- row 1) 0)
        bottom (min (+ row 1) (- rows 1))]
    (->> (for [c (range left (+ right 1))
               r (range top (+ bottom 1))]
           [r c])
         (filter #(not= % coord))
         (map #(get-cell layout %)))))

(defn occupied
  ([layout]
   (->> (:seats layout)
        (filter #(= :# %))
        count))
  ([layout coord]
   (->> (adjacency layout coord)
        (filter #(= :# %))
        count)))

(defn update-one [seat occupied]
  (cond
    (and (= seat :L) (= occupied 0)) :#
    (and (= seat :#) (>= occupied 5)) :L
    :else seat))

(defn adj [seats rows cols row col]
  (let [offset (+ (* cols row) col)

        cc (- cols 1)
        rr (- rows 1)

        topidx (- offset cols)
        botidx (+ offset cols)

        lt (if (or (= row 0) (= col 0)) 0 (if (= :# (nth seats (- topidx 1))) 1 0))
        ct (if (= row 0) 0 (if (= :# (nth seats topidx)) 1 0))
        rt (if (or (= row 0) (= col cc)) 0 (if (= :# (nth seats (+ topidx 1))) 1 0))

        lm (if (= col 0) 0 (if (= :# (nth seats (- offset 1))) 1 0))
        cm 0  ;; (nth seats offset)
        rm (if (= col cc) 0 (if (= :# (nth seats (+ offset 1))) 1 0))

        lb (if (or (= col 0) (= row rr)) 0 (if (= :# (nth seats (- botidx 1))) 1 0))
        cb (if (= row rr) 0 (if (= :# (nth seats botidx)) 1 0))
        rb (if (or (= col cc) (= row rr)) 0  (if (= :# (nth seats (+ botidx 1))) 1 0))]
    (+ lt ct rt lm cm rm lb cb rb)))

(defn first-meet [seats cols coords]
  (->> coords
       (map (fn [[row col]]
              (nth seats (+ (* row cols) col))))
       (drop-while #(= % :.))
       first))

(first-meet [:. :. :L :#] 4 [[0 0] [0 1] [0 2] [0 3]])

(defn adj2 [seats rows cols row col]
  (let [d1 (map vector (map dec (range row 0 -1)) (map dec (range col 0 -1)))
        d2 (map #(vector % col) (map dec (range row 0 -1)))
        d3 (map vector (map dec (range row 0 -1)) (range (inc col) cols))
        d4 (map #(vector row %) (range (inc col) cols))
        d5 (map vector (range (inc row) rows) (range (inc col) cols))
        d6 (map #(vector % col) (range (inc row) rows))
        d7 (map vector (range (inc row) rows) (map dec (range col 0 -1)))
        d8 (map #(vector row %) (map dec (range col 0 -1)))
        adjs [(first-meet seats cols d1)
              (first-meet seats cols d2)
              (first-meet seats cols d3)
              (first-meet seats cols d4)
              (first-meet seats cols d5)
              (first-meet seats cols d6)
              (first-meet seats cols d7)
              (first-meet seats cols d8)]]
    (->> adjs
         (filter #(= :# %))
         count)))

(defn occupied-map [{:keys [seats rows cols]}]
  (println "occupied-map")
  (for [r (range rows)
        c (range cols)]
    (adj2 seats rows cols r c)))

(defn update-occ-map [layout]
  (println "update-occ-map")
  (->> layout
       occupied-map
       (map update-one (:seats layout))
       (assoc layout :seats)))

(defn update-cell [layout coord]
  (let [seat (get-cell layout coord)
        occupied (occupied layout coord)]
    (update-one seat occupied)))

(defn update-layout [layout]
  (let [rows (:rows layout)
        cols (:cols layout)
        coords (for [r (range rows) c (range cols)] [r c])]
    (->> coords
         (map #(update-cell layout %))
         (assoc layout :seats))))

(defn layout-changed? [{s1 :seats} {s2 :seats}]
  (->> (map not= s1 s2)
       (some identity)))

(defn evolve [layout]
  (->> layout
       (iterate update-occ-map)
       (partition 2 1)
       (drop-while (fn [[l1 l2]] (layout-changed? l1 l2)))
       first
       first))

(defn load-seq [lines]
  (let [rows (count lines)
        cols (count (first lines))]
    (->> lines
         (apply concat)
         (map #(keyword (str %)))
         (create-layout rows cols))))

(defn load-file [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         load-seq)))

(def d1 ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....")

(defn load-string [s]
  (->> s
       str/split-lines
       load-seq))

(defn run-very-slow-part1 [filename]
  (let [evolved (-> filename
                    load-file
                    evolve)]
    evolved))

;; (def p2-evolved (run-very-slow-part1 F))
;; (print p2-evolved)
;; (occupied p2-evolved) answer 1863
(def small-data "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
")

(->> (load-string small-data)
     evolve
     occupied)
