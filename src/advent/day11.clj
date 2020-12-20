(ns advent.day11
  (:require [clojure.java.io :as io]))

(def F "resources/day11-input.txt")

(defn create-layout [rows cols seats]
  {:seats seats :rows rows :cols cols})

(defn get-cell [layout [row col]]
  (let [pos (+ (* row (:cols layout))
               col)]
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
    (and (= seat :#) (>= occupied 4)) :L
    :else seat))

(defn adj [seats rows cols row col]
  (let [offset (+ (* cols row) col)

        cc (- cols 1)
        rr (- rows 1)

        topidx (- offset cols)
        botidx (+ offset cols)

        lt (if (or (= row 0) (= col 0)) 0 (nth seats (- topidx 1)))
        ct (if (= row 0) 0 (nth seats topidx))
        rt (if (or (= row 0) (= col cc)) 0 (nth seats (+ topidx 1)))

        lm (if (= col 0) 0  (nth seats (- offset 1)))
        cm 0  ;; (nth seats offset)
        rm (if (= col cc) 0 (nth seats (+ offset 1)))

        lb (if (or (= col 0) (= row rr)) 0 (nth seats (- botidx 1)))
        cb (if (= row rr) 0  (nth seats botidx))
        rb (if (or (= col cc) (= row rr)) 0  (nth seats (+ botidx 1)))]
    (+ lt ct rt lm cm rm lb cb rb)))


(defn occupied-map [{:keys [seats rows cols]}]
  (println "occupied-map")
  (let [seats (map #(if (= % :#) 1 0) seats)]
    (for [r (range rows)
          c (range cols)]
      (adj seats rows cols r c))))

(occupied-map (create-layout 3 3 [:# :# :#
                                  :L :# :L
                                  :# :# :#]))

(defn update-occ-map [layout]
  (println "update-occ-map")
  (assoc layout :seats
         (map update-one (:seats layout) (occupied-map layout))))

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
         (create-layout rows cols))))

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

(defn load-data [filename]
  (with-open [rdr (io/reader filename)]
    (let [ls (->> rdr
                  line-seq
                  vec)
          rows (count ls)
          cols (count (first ls))]
      (->> ls
           (apply concat)
           (map #(keyword (str %)))
           (create-layout rows cols)))))

(def input1-rows 10)
(def input1-cols 10)
(def input1-1 "L.LL.LL.LLLLLLLLL.LLL.L.L..L..LLLL.LL.LLL.LL.LL.LLL.LLLLL.LL..L.L.....LLLLLLLLLLL.LLLLLL.LL.LLLLL.LL")
(def input1-2 "#.##.##.#########.###.#.#..#..####.##.###.##.##.###.#####.##..#.#.....###########.######.##.#####.##")
(def input1-3 "#.LL.L#.###LLLLLL.L#L.L.L..L..#LLL.LL.L##.LL.LL.LL#.LLLL#.##..L.L.....#LLLLLLLL##.LLLLLL.L#.#LLLL.##")

(def input11-layout
  (->> input1-1
       (map #(keyword (str %)))
       (create-layout input1-rows input1-cols)))

(def input12-layout
  (->> input1-2
       (map #(keyword (str %)))
       (create-layout input1-rows input1-cols)))

(def input13-layout
  (->> input1-3
       (map #(keyword (str %)))
       (create-layout input1-rows input1-cols)))

(->> input11-layout
     update-layout
     update-layout
     )

(->> input11-layout
     evolve
     occupied)

(def evolved (-> F
             load-data
             evolve))


(println)

(occupied evolved)
