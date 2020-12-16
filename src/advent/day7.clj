(ns advent.day7
  (:require [clojure.java.io :as io]
            [clojure.string :as str]))

(def F "resources/day7-input")

(def re-rule-bag #"^(\w+ \w+) bags contain ")

(def re-rule-contents #"(\d+) (\w+ \w+) bags?[,.]")

(defn parse-rule [str-rule]
  (let [[_ bag] (re-find re-rule-bag str-rule)
        contents (->> (re-seq re-rule-contents str-rule)
                      (map (fn [[_ n v]] [v (Integer/parseInt n)]))
                      (into {}))]
    (hash-map bag contents)))


(defn load-data [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         (map parse-rule)
         (into {}))))


(defn can-contain? [rules bag content]
  (let [bag-contents (rules bag)]
    (if (contains? bag-contents content)
      true
      (some #(can-contain? rules % content) (keys bag-contents)))))


(let [rules (load-data F)]
  (->> (filter #(can-contain? rules % "shiny gold")
               (remove #(= "shiny gold" %)
                       (keys rules)))
       count))


(defn bags [rules bag]
  (let [contents (rules bag)]
    (if (empty? contents)
      1
      (inc (apply + (map (fn [[k v]] (* (bags rules k) v)) contents))))))

(dec (bags (load-data F) "shiny gold"))

(def example "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.")

(let [rule (->> example
                str/split-lines
                (map parse-rule)
                (into {}))]
  (dec (bags rule "shiny gold")))

(bags {"a" {"x" 3 "y" 4} "x" {"s" 10} "y" {} "s" {}} "a")
