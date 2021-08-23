(ns advent.y2020.day13
  (:require [clojure.string :as str]))

(first
 (drop-while #(<= % 939)
             (iterate #(+ 7 %) 0)))

(def ids [7 13 59 31 19])

(defn p1 [depart ids]
  (->> ids
       (map (fn [n] (->> (iterate #(+ n %) 0)
                         (drop-while #(<= % depart))
                         first
                         (vector n))))
       (reduce (fn [[_ acc :as old] [k new]] (if (< new acc) [k new] old)))
       ((fn [[id ts]] (* id (- ts depart))))))


(def depart 1000495)

(def ids (str/split
          "19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,521,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,x,17,x,x,x,x,x,x,x,x,x,x,x,29,x,523,x,x,x,x,x,37,x,x,x,x,x,x,13"
          #","))

(defn part1 [depart ids]
  (->> ids
       (filter #(not= "x" %))
       (map #(Integer/parseInt %))
       (p1 depart)))

(part1 depart ids)

(defn time-table [ids]
  (let [size (count ids)]
    (->> ids
         (map vector (range size))
         (filter #(not= "x" (second %)))
         (map (fn [[delay id]]
                (let [depart (Integer/parseInt id)]
                  [(- (mod delay depart)) depart])))
         (sort-by first >)
         ;; (map (fn [[start depart]] (iterate #(+ depart %) start)))
         )))

(defn jump [[s1 _] [s2 j2]]
  (let [d (- s1 s2)
        m (mod d j2)
        q (quot d j2)]
    (if (zero? m)
      [s1 j2]
      [(+ s2 (* j2 (+ q 1))) j2])))

(jump [5 3] [-1 7])

(defn run-all [runners]
  (loop [runners runners]
    (let [fastest (first runners)
          followers (map (fn [other] (jump fastest other)) (rest runners))
          new-runners (conj followers fastest)
          all-ts (map first new-runners)]
      (if (apply = all-ts)
        (first all-ts)
        (recur (sort-by first > new-runners))))))

(defn time-table2 [ids]
  (let [size (count ids)]
    (->> ids
         (map vector (range size))
         (filter #(not= "x" (second %)))
         (map (fn [[depart id]]
                [depart (Integer/parseInt id)])))))

(defn jump2 [[s p] [idx num]]
  (loop [s s]
    (if (zero? (mod (+ s idx) num))
      [s (* p num)]
      (recur (+ s p)))))

(def small-ids (str/split "17,x,13,19" #","))

(->> ids
     time-table2
     (reduce jump2)
     first)
