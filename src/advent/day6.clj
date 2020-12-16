(ns advent.day6
  (:require [clojure.java.io :as io]
            [clojure.string :as str]))

(def F "resources/day6-input")

(defn load-data [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         (partition-by empty?)
         (remove #(str/blank? (first %)))
         vec)))

(defn count-question1 [group]
  (->> group
       (apply concat)
       set
       count))

(defn solve [filename counter]
  (->> filename
       load-data
       (map (fn [lst] (map set lst)))
       (map counter)
       (apply +)))

(solve F count-question1)

(defn count-question2 [group]
  (->> group
       first
       (filter (fn [q] (every? #(contains? % q) group)))
       count))

(solve F count-question2)
