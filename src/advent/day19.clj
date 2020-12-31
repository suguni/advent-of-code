(ns advent.day19
  (:require [clojure.string :as str]
            [clojure.set :as set]))

(def example "
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
")

(defn parse-seq [seqs]
  (->> seqs
       (re-seq #"\d+")
       (map #(Integer/parseInt %))
       vec))

(defn parse-rule [line]
  (let [[id pattern] (str/split line #":")
        id (Integer/parseInt id)]
    (cond
      (str/includes? pattern "\"") [id (re-find #"\w" pattern)]
      (str/includes? pattern "|") [id (->> (str/split pattern #"\|")
                                           (map parse-seq)
                                           set)]
      :else [id (parse-seq pattern)])))

(defn load-data [input]
  (let [[book messages] (-> input
                            (str/split #"\n\n"))
        book (->> book
                  str/trim
                  str/split-lines
                  (map parse-rule)
                  (into {}))
        messages (->> messages
                      str/trim
                      str/split-lines
                      set)]
    [book messages]))

(defn sub-rule [pattern book]
  (cond
    (string? pattern) pattern
    (number? pattern) (sub-rule (book pattern) book)
    (vector? pattern) (->> pattern
                           (map #(sub-rule % book))
                           (reduce (fn [acc ss]
                                     (if (string? ss)
                                       (map #(str % ss) acc)
                                       (for [a acc
                                             s ss] (str a s)))) [""]))
    (set? pattern) (->> pattern
                        (map #(sub-rule % book))
                        flatten)))


;; (defn sub-rule2 [patterns book]
;;   (if (seq patterns)
;;     (let [pattern (peek patterns)
;;           patterns (pop patterns)]

;;       (cond
;;         (string? pattern) )
;;       )

;;     book))


(def filename "resources/day19-input.txt")

(let [[b m] (load-data example)]
  (def book b)
  (def messages m))

(defn solve [book messages]
  (->> book
       (sub-rule 0)
       ;; (map #(.length %))
       ;; (apply max)
       set
       (set/intersection messages)
       count
       ))

(solve book messages)
