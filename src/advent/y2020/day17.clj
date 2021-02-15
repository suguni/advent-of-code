(ns advent.y2020.day17
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

(defn pad-hcube [[sx sy sz _ :as dim] hcube n]
  (let [[psx psy psz _] (map #(+ % (* n 2)) dim)
        cube (repeat-str (* psx psy psz n) \.)
        padded-hcube (->> hcube
                          (partition-str (* sx sy sz))
                          (map #(pad-cube [sx sy sz] % n))
                          str/join)]
    (str cube padded-hcube cube)))

(pad-hcube [1 1 1 1] "x" 1)

(defn cube-neighbors [[sx sy sz]]
  (for [z [-1 0 1]
        y [-1 0 1]
        x [-1 0 1]
        :let [r (+ (* z (* sx sy)) (* y sx) x)]
        :when (not (zero? r))]
    r))

(defn hcube-neighbors [[sx sy sz sw]]
  (for [w [-1 0 1]
        z [-1 0 1]
        y [-1 0 1]
        x [-1 0 1]
        :let [r (+ (* w (* sx sy sz)) (* z (* sx sy)) (* y sx) x)]
        :when (not (zero? r))]
    r))

(defn cube-coords [[sx sy sz]]
  (for [z (range 1 (dec sz))
        y (range 1 (dec sy))
        x (range 1 (dec sx))]
    (+ (* z (* sx sy)) (* y sx) x)))

(defn hcube-coords [[sx sy sz sw]]
  (for [w (range 1 (dec sw))
        z (range 1 (dec sz))
        y (range 1 (dec sy))
        x (range 1 (dec sx))]
    (+ (* w (* sx sy sz)) (* z (* sx sy)) (* y sx) x)))

(defn generate [dim cube]
  (let [padded (pad-hcube dim cube 2)    ; 7x7x7
        padded-dim (map #(+ 4 %) dim)
        coords (hcube-coords padded-dim)
        neighbors (hcube-neighbors padded-dim)
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
  (let [st (System/currentTimeMillis)]
    (loop [input (.replaceAll input "\\s+" "")
           dim dim
           i 0
           t st]

      (println (str "[" i "] " (- t st) "ms " dim))

      (if (= i n)
        (->> input
             (filter #(= % \#))
             count)
        (recur (generate dim input)
               (mapv #(+ % 2) dim)
               (inc i)
               (System/currentTimeMillis))))))


(vector [1 2 3])
;; (evolve [3 3 1 1] ".#...####" 6)

;; (evolve [3 3 3] e 5)
;; (evolve [8 8 1] s 6)

;;;  213
24
