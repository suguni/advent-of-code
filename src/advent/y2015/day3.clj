(ns advent.y2015.day3
  (:require [clojure.set :as set]))

(defn step [[cx cy] dir]
  (case dir
    \^ [cx (+ cy 1)]
    \v [cx (- cy 1)]
    \> [(+ cx 1) cy]
    \< [(- cx 1) cy]))

(defn move [dirs]
  (reduce
    (fn [acc dir]
      (let [start (last acc)]
        (conj acc (step start dir))))
    [[0 0]]
    dirs))

(defn load-data [filename]
  (slurp filename))

(def F "resources/2015/day3-input.txt")

(defn day3-p1 []
  (->> F
       load-data
       move
       set
       count))

(comment
  (day3-p1))

(defn pair-move [dirs]
  (let [ps (partition 2 dirs)
        ds1 (map first ps)
        ds2 (map second ps)]
    [(move ds1) (move ds2)]))

(defn day3-p2 []
  (let [[p1 p2] (->> F
                     load-data
                     pair-move)
        s1 (set p1)
        s2 (set p2)]
    (->> (set/union s1 s2)
         count)))

(comment
  (day3-p2))