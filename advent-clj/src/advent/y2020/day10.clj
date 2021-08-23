(ns advent.y2020.day10
  (:require [clojure.java.io :as io]))

(def F "resources/2020/day10-input.txt")

(defn load-data [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         (map #(Integer/parseInt %))
         vec)))

;; 어댑터 : 입력은 항상 자기의 출력보다 1, 2, 3 볼트 낮게 받아서 자신의 고유 출력 볼트를 낸다.
;; 장비 : 가방에 있는 어뎁터들 중 가장 큰것보다 3볼트 높다.
;; 가방안에 있는 모든 어댑터를 연결했을때 3볼트 차이나는 것과 1볼트 차이나는 것의 갯수의 곱은?

(def long-data (->> F load-data))

(def input1 [16 10 15 5 1 11 7 19 6 12 4])

(def input2 [28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39 11
             1 32 25 35 8 17 7 9 4 2 34 10 3])

(defn diffs [numbers]
  (->> (apply max numbers)
       (+ 3)
       (conj numbers 0)
       sort
       (partition 2 1)
       (map (fn [[a b]] (- b a)))))

(defn part1 [data]
  (->> data
       diffs
       frequencies))

(defn solve-part1 [filename]
  (->> filename
       load-data
       part1
       vals
       (apply *)))

(defn under-3-pair [v as]
  (->> (range 1 (inc (count as)))
       (map #(let [[h r] (split-at % as)]
               [(last h) (vec r)]))
       (take-while (fn [[h _]] (<= (- h v) 3)))
       vec))

(under-3-pair 0 [1 3 4 5])

(split-at 6 [1 2 3 4 5])

(defn slow-arrangement [v as]
  (if (empty? as)
    [[v]]
    (->> (under-3-pair v as)
         (map (fn [[v1 as1]] (slow-arrangement v1 as1)))
         (apply concat)
         (map #(concat [v] %)))))

(comment
  (def m-arrangement
    (fn [v as]
      (if (empty? as)
        [[v]]
        (->> (under-3-pair v as)
             (map (fn [[v1 as1]] (m-arrangement v1 as1)))
             (apply concat)
             (map #(concat [v] %)))))))


(defn arr [diffs]
  (if (<= (count diffs) 1)
    1
    (let [f (first diffs)
          s (second diffs)]
      (if (> (+ f s) 3)
        (arr (rest diffs))
        (+ (arr (rest diffs))
           (arr (concat [(+ f s)] (rest (rest diffs)))))))))


(arr (diffs input1))

(def m-arr (memoize
            (fn [diffs]
              (if (<= (count diffs) 1)
                1
                (let [f (first diffs)
                      s (second diffs)]
                  (if (> (+ f s) 3)
                    (m-arr (rest diffs))
                    (+ (m-arr (rest diffs))
                       (m-arr (concat [(+ f s)] (rest (rest diffs)))))))))))

(m-arr (diffs long-data))
