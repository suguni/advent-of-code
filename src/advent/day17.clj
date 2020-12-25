(ns advent.day17
  (:require
            [clojure.string :as str]))

(defn repeat-str [n s]
  (->> s
       (repeat n)
       (apply str)))

(defn partition-str [n s]
  (->> s
       (partition n)
       (map #(apply str %))))

(def s "
..##.#.#
.#####..
#.....##
##.##.#.
..#...#.
.#..##..
.#...#.#
#..##.##
")

(def e "
#..
..#
.#.

#.#
.##
.#.

#..
..#
.#.
")

(defn remove-sps [s]
  (.replaceAll s "\\s+" ""))

(defn update-cell [state active-neighbors]
  (case state
    \# (if (or (= 2 active-neighbors)
               (= 3 active-neighbors)) \# \.)
    \. (if (= active-neighbors 3) \# \.)))

(defn pad-plane [[sx _] slice n]
  (let [pad-line (->> \.
                      (repeat-str (+ sx (* n 2)))
                      (repeat-str n))
        pad-side (repeat-str n \.)
        padded-slice (->> slice
                          (partition-str sx)
                          (map #(str pad-side % pad-side))
                          str/join)]
    (str pad-line padded-slice pad-line)))

(defn pad-cube [[sx sy _ :as dim] cube n]
  (let [[psx psy _] (map #(+ % (* n 2)) dim)
        plane (repeat-str (* psx psy n) \.)
        padded-cube (->> cube
                         (partition-str (* sx sy))
                         (map #(pad-plane [sx sy] % n))
                         str/join)]
    (str plane padded-cube plane)))

(defn cube-neighbors [[sx sy _]]
  (let [stride (* sx sy)
        z=0 (for [r [-1 0 1]
                  c [-1 0 1]] (+ (* r sx) c))
        z-1 (map #(- % stride) z=0)
        z+1 (map #(+ % stride) z=0)]
    (filter #(not (zero? %)) (concat z-1 z=0 z+1))))

(defn cube-coords [[sx sy sz]]
  (for [z (range 1 (dec sz))
        y (range 1 (dec sy))
        x (range 1 (dec sx))]
    (+ (* z (* sx sy)) (* y sx) x)))

(defn generate [dim cube]
  (let [padded (pad-cube dim cube 2)    ; 7x7x7
        padded-dim (map #(+ 4 %) dim)
        coords (cube-coords padded-dim)
        neighbors (cube-neighbors padded-dim)
        active-counts (fn [i] (->> neighbors
                                   (filter #(= \# (.charAt padded (+ % i))))
                                   ;; (map #(+ i %))
                                   count))]
    (->> coords
         (map (fn [c]
                (let [cell (.charAt padded c)
                      actives (active-counts c)]
                  (update-cell cell actives))))
         (apply str))))

(defn evolve [dim input n]
  (loop [input (.replaceAll input "\\s+" "")
         dim dim
         i 0]
    (if (= i n)
      (->> input
           (filter #(= % \#))
           count)
      (recur (generate dim input)
             (map #(+ % 2) dim)
             (inc i)))))

(evolve [3 3 3] e 5)
(evolve [8 8 1] s 6)

;;;  213
