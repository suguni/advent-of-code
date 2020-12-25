(ns advent.day17
  (:require
            [clojure.string :as str]))

(def input (str (repeat-str (* 8 8 3) \.) (.replaceAll "
..##.#.#
.#####..
#.....##
##.##.#.
..#...#.
.#..##..
.#...#.#
#..##.##
" "\\s+" "") (repeat-str (* 8 8 4) \.)))

(def c0 "..........#...####.........")
(def c1 "#....#.#.#.#.##.#.#....#.#.")
(def c2 (.replaceAll "
.....
.....
..#..
.....
.....

..#..
.#..#
....#
.#...
.....

##...
##...
#....
....#
.###.

..#..
.#..#
....#
.#...
.....

.....
.....
..#..
.....
.....
" "\\s+" ""))


(defn update-cell [state active-neighbors]
  (case state
    \# (if (or (= 2 active-neighbors)
               (= 3 active-neighbors)) \# \.)
    \. (if (= active-neighbors 3) \# \.)))

(update-cell \. 3)

(defn repeat-str [n s]
  (->> s
       (repeat n)
       (apply str)))

(defn partition-str [n s]
  (->> s
       (partition n)
       (map #(apply str %))))

(defn pad-slice [slice size n]
  (let [pad-line (->> \.
                      (repeat-str (+ size (* n 2)))
                      (repeat-str n))
        pad-side (repeat-str n \.)
        padded-slice (->> slice
                          (partition-str size)
                          (map #(str pad-side % pad-side))
                          str/join)]
    (str pad-line padded-slice pad-line)))

(defn pad-cube [cube size n]
  (let [padded-size (+ size (* n 2))
        plane (repeat-str (* padded-size padded-size n) \.)
        padded-cube (->> cube
                         (partition-str (* size size))
                         (map #(pad-slice % size n))
                         str/join)]
    (str plane padded-cube plane)))

(pad-cube "x" 1 2)
(pad-slice "....*...." 3 2)

(defn cube-neighbors [size]
  (let [stride (* size size)
        z=0 (for [r [-1 0 1]
                  c [-1 0 1]] (+ (* r size) c))
        z-1 (map #(- % stride) z=0)
        z+1 (map #(+ % stride) z=0)]
    (filter #(not (zero? %)) (concat z-1 z=0 z+1))))

(cube-neighbors 5)

(defn cube-size [cube]
  (Math/round (Math/pow (.length cube) (/ 1 3))))

(defn cube-coords [size]
  (let [stride (* size size)]
    (for [z (range 1 (dec size))
          y (range 1 (dec size))
          x (range 1 (dec size))]
      (+ (* z stride) (* y size) x))))

(defn generate [cube]
  (let [size (cube-size cube)
        padded (pad-cube cube size 2)   ; 7x7x7
        coords (cube-coords (+ size 2 2))
        neighbors (cube-neighbors (+ size 2 2))
        active-counts (fn [i] (->> neighbors
                                   (filter #(= \# (.charAt padded (+ % i))))
                                   (map #(+ i %))
                                   count))]
    (->> coords
         (map (fn [c]
                (let [cell (.charAt padded c)
                      actives (active-counts c)]
                  (update-cell cell actives))))
         (apply str))))

(->> input
     generate
     generate
     generate
     generate
     generate
     generate
     ;; .length
     (filter #(= % \#))
     count)

;;;  213


(pad-cube c1 3 0)
"
...
...
...

...
.#.
...

..#
...
#..

...
...
.#.

#...##...#.........#......#...#.............."

"
.......
.......
.......
.......
.......
.......
.......

.......
.#.....
.#...#.
.......
.......
..#.#..
.##...#

.......
.......
...#...
...#...
#......
.......
.......

......................................"
