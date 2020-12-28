(ns advent.day19
  (:require [clojure.string :as str]
            [clojure.set :as set]))

(def example-input "
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"
")

(defn parse-seq [seqs]
  (->> seqs
       (re-seq #"\d+")
       (map #(Integer/parseInt %))
       vec))

(defn parse-line [line]
  (let [[id pattern] (str/split line #":")
        id (Integer/parseInt id)]
    (cond
      (str/includes? pattern "\"") [id (re-find #"\w" pattern)]
      (str/includes? pattern "|") [id (->> (str/split pattern #"\|")
                                           (map parse-seq)
                                           set)]
      :else [id (parse-seq pattern)])))

(assoc {} :a (inc (get {} :a 0)))

(update {} :a inc)


(defn sub-rule
  ([pattern book]
   (sub-rule pattern book {}))

  ([pattern book depth]
   (cond
     (string? pattern) pattern
     (number? pattern) (sub-rule (book pattern) book depth)
     (vector? pattern) (->> pattern
                            (map #(sub-rule % book depth))
                            (reduce (fn [acc ss]
                                      (if (string? ss)
                                        (map #(str % ss) acc)
                                        (for [a acc
                                              s ss] (str a s)))) [""]))
     (set? pattern) (->> pattern
                         (map #(sub-rule % book depth))
                         flatten))))



;; (defn sub-rule2 [patterns book]
;;   (if (seq patterns)
;;     (let [pattern (peek patterns)
;;           patterns (pop patterns)]

;;       (cond
;;         (string? pattern) )
;;       )

;;     book))



(def example (->> example-input
                  .trim
                  str/split-lines
                  (map parse-line)
                  (into {})))


(def filename "resources/day19-input.txt")


(defn solve []
  (let [[book messages] (-> filename
                            slurp
                            (str/split #"\n\n"))
        book (-> (->> book
                      str/split-lines
                      (map parse-line)
                      (into {}))
                 (assoc 8 #{[42] [42 8]})
                 (assoc 11 #{[42 31] [42 11 31]})
                 )

        messages (->> messages
                      str/split-lines
                      set)]
    (->> book
         (sub-rule 0)
         ;; (map #(.length %))
         ;; (apply max)
         set
         (set/intersection messages)
         count
         ))
  )


;; (solve)

(set/intersection
 (set (sub-rule 0 example))
 #{"ababbb"
   "bababa"
   "abbbab"
   "aaabbb"
   "aaaabbb"})

(sub-rule 1 {0 [4 1]
             1 #{[4 5] [5 4]}
             4 "a"
             5 "b"})
