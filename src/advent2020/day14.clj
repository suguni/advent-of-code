(ns advent2020.day14
  (:require [clojure.string :as str]
            [clojure.java.io :as io]))

(defn padded [num]
  (let [s (str
           (format "%036d" 0)
           (Integer/toString num 2))
        c (.length s)]
    (subs s (- c 36))))

(defn parse-binary [binary]
  (Long/parseLong binary 2))

(defn masking [mask value]
  (->> (padded value)
       (map (fn [m v] (if (= m \X) v m)) mask)
       (apply str)))

(def memory {})

(def inst1 [8 11])

(defn run [memory mask ins]
  (reduce (fn [memory [loc value]]
            (assoc memory loc (parse-binary (masking mask value))))
          memory
          ins))

(masking "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0" 3)

(run {} "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X" [[8 11] [7 101] [8 0]])

(def F "resources/2020/day14-input.txt")

(def instructions [])

(def source (str/split-lines "mask = 0010X01001X010000110100000X000010X11
mem[41717] = 288
mem[54146] = 1656
mem[30135] = 4799584
mask = 01X10101X11X01XX01X000011X1000110110
mem[29142] = 13227025
mem[32455] = 1814
mem[42522] = 385316684
mem[29904] = 5334
mem[51087] = 1492"))

(defn parse-mask-line [line]
  (->> line
       (re-find #"mask = (.+)")
       second))

(defn parse-instruction-lines [lines]
  (->> lines
       (map (fn [line]
              (->> line
                   (re-find #"mem\[(.+)\] = (.+)")
                   rest
                   (map #(Integer/parseInt %))
                   vec)))))

(defn parse-block [block]
  (let [mask (parse-mask-line (first block))
        ins (parse-instruction-lines (rest block))]
    {:mask mask :instructions ins}))

(defn parse-source [lines]
  (->> lines
       (partition-by #(.startsWith % "mask"))
       (partition 2)
       (map #(apply concat %))
       (map parse-block)))

;; (parse-source source)

(defn part1 [source]
  (->> source
       parse-source
       (reduce (fn [memory {:keys [mask instructions]}]
                 (run memory mask instructions)) {})
       (map second)
       (apply +)))

;; (with-open [rdr (io/reader F)]
;;   (->> rdr
;;        line-seq
;;        part1))

(defn count-x [mask]
  (->> mask
       (filter #(= % \X))
       count))

(defn combinations [n]
  (loop [n n
         c [[]]]
    (if (zero? n)
      c
      (recur (dec n)
             (->> c
                  (map (fn [c] [(conj c 0) (conj c 1)]))
                  (apply concat))))))

(defn replace-x [mask bits]
  (loop [bits bits
         mask mask]
    (if (empty? bits)
      mask
      (recur (rest bits)
             (.replaceFirst mask "X" (str (first bits)))))))

(defn mask-combs [mask]
  (->> mask
       count-x
       combinations
       (map #(replace-x mask %))))

(defn masking2 [mask value]
  (->> (padded value)
       (map (fn [m v] (if (= m \0) v m)) mask)
       (apply str)))

(masking2 "000000000000000000000000000000X1001X" 42)

(defn run-fluctuation [memory mask ins]
  (reduce (fn [memory [loc value]]
            (->> loc
                 (masking2 mask)
                 (mask-combs)
                 (map parse-binary)
                 (reduce (fn [memory loc] (assoc memory loc value))
                         memory)))
          memory
          ins))

(defn part2 [source]
  (->> source
       parse-source
       (reduce (fn [memory {:keys [mask instructions]}]
                 (run-fluctuation memory mask instructions)) {})
       (map second)
       (apply +)))

(def source2 (str/split-lines "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"))

;; (with-open [rdr (io/reader F)]
;;   (->> rdr
;;        line-seq
;;        part2))
