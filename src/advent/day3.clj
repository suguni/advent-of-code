(ns advent.day3
  (:require [clojure.string :as str]))

(def FILE "resources/day3-input")

(defn load-tile [filename]
  (-> filename
      snnlurp
      str/split-lines))

(defn slope-coords [[right down] [width height]]
  (->> (iterate (fn [[col row]] [(+ col right) (+ row down)]) [0 0])
       (map (fn [[col row]] [row (mod col width)]))
       (take-while (fn [[row _]] (< row height)))
       rest))

(def tile (load-tile FILE))

(defn tree-encounter [tile coords]
  (->> coords
       (map #(get-in tile %))
       (filter #(= \# %))
       count))

(defn tree-encounter-at [filename slope]
  (let [tile (load-tile filename)
        height (count tile)
        width (count (get tile 0))
        coords (slope-coords slope [width height])]
    (tree-encounter tile coords)))

(tree-encounter-at "resources/day3-input" [3 1])

(->> [[1 1] [3 1] [5 1] [7 1] [1 2]]
    (map #(tree-encounter-at FILE %))
    (apply *))
