(ns advent.day6
  (:require [clojure.java.io :as io]
            [clojure.string :as str]))

(def F "resources/day6-input")

(defn parse [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         (reduce (fn [acc line]
                   (if (str/blank? line)
                     (conj acc [])
                     (conj (pop acc) (conj (last acc) (set line)))))
                 [[]]))))

(defn count-question1 [group]
  (->> group
       (apply concat)
       set
       count))

(defn solve [filename counter]
  (->> filename
       parse
       (map counter)
       (apply +)))

(solve F count-question1)

(defn count-question2 [group]
  (->> group
       first
       (filter (fn [q] (every? #(contains? % q) group)))
       count))

(solve F count-question2)
