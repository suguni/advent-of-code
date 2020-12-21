(ns advent.day12
  (:require [clojure.java.io :as io]
            [clojure.string :as str]))

(def input "F10
N3
F7
R90
F11")

(def boat {:east 0 :north 0 :dir 90
            :we 10 :wn 1 })

(defn forward [boat value]
  (case (:dir boat)
    0 (update boat :north #(+ % value))
    90 (update boat :east #(+ % value))
    180 (update boat :north #(- % value))
    270 (update boat :east #(- % value))))

(defn move [boat [action value]]
  (case action
    :N (update boat :north #(+ % value))
    :S (update boat :north #(- % value))
    :E (update boat :east #(+ % value))
    :W (update boat :east #(- % value))
    :R (update boat :dir #(mod (+ % value) 360))
    :L (update boat :dir #(mod (+ % (- 360 value)) 360))
    :F (forward boat value)))

(defn forward2 [{:keys [east north we wn] :as boat} value]
  (let [new-east (* we value)
        new-north (* wn value)]
    (assoc boat
           :east (+ new-east east)
           :north (+ new-north north))))

(defn rotate2 [{:keys [we wn] :as boat} value]
  (let [[new-we new-wn] (case value
                          0 [we wn]
                          90 [wn (- we)]
                          180 [(- we) (- wn)]
                          270 [(- wn) we])]
    (assoc boat :we new-we :wn new-wn)))

(-> boat
    (rotate2 90)
    (rotate2 90))

(defn move2 [boat [action value]]
  (case action
    :N (update boat :wn #(+ % value))
    :S (update boat :wn #(- % value))
    :E (update boat :we #(+ % value))
    :W (update boat :we #(- % value))
    :R (rotate2 boat value)
    :L (rotate2 boat (- 360 value))
    :F (forward2 boat value)))

(def actions [[:F 10] [:N 3] [:F 7] [:R 90] [:F 11]])
(reduce move2 boat actions)
(move2 boat [:L 270])
(move2 boat [:F 10])

(-> boat
    (move2 [:F 10])
    (move2 [:N 3])
    (move2 [:F 7])
    (move2 [:R 90]))

(def F "resources/day12-input.txt")

(defn parse-line [line]
  (let [action (.substring line 0 1)
        value (.substring line 1)]
    [(keyword (str action))
     (Integer/parseInt (str value))]))

(with-open [rdr (io/reader F)]
  (->> rdr
       line-seq
       (map parse-line)
       (reduce move2 boat)))
