(ns advent2020.day8
  (:require [clojure.java.io :as io]))


(def F "resources/2020/day8-input.txt")

(def init-state { :PC 0 :A 0 })

(def program
  [[:nop 0]
   [:acc 1]
   [:jmp 4]
   [:acc 3]
   [:jmp -3]
   [:acc -99]
   [:acc 1]
   [:jmp -4]
   [:acc 6]])

(defn run [state ins op]
  (case ins
    :nop (update state :PC inc)
    :acc (-> state
             (update :A #(+ op %))
             (update :PC inc))
    :jmp (update state :PC #(+ op %))))

(defn run-tick [{:keys [PC _] :as state} program]
  (if (>= PC (count program))
    state
    (let [[ins op] (nth program PC)]
      (run state ins op))))

(defn go [state program]
  (iterate #(run-tick % program) state))

(defn go-and-halt [program]
  (let [program-size (count program)]
    (->> (go init-state program)
         (reductions (fn [[acc _] {PC :PC :as state}]
                       [(update acc (min PC program-size) inc) state])
                     [(into [] (repeat (inc program-size) 0)) init-state])
         (drop-while (fn [[acc state]]
                       (and
                        (zero? (nth acc program-size))
                        (every? #(< % 2) acc))))
         first)))

(def R #"(acc|jmp|nop) ([+|-]?\d+)")

(defn parse-line [s]
  (when-let [[_ ins op] (re-matches R s)]
    [(keyword ins) (Integer/parseInt op)]))

(parse-line "acc +8")

(defn load-code [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         (map parse-line)
         vec)))

(defn is-inf? [[acc _]]
  (zero? (last acc)))

(defn is-terminated? [[acc _ ]]
  (not (zero? (last acc))))

(->> F
     load-code
     go-and-halt
     is-inf?)

(def terminating-program
  [[:nop 0]
   [:acc 1]
   [:jmp 4]
   [:acc 3]
   [:jmp -3]
   [:acc -99]
   [:acc 1]
   [:acc 6]])

(take 8 (go init-state terminating-program))

(->> terminating-program
     go-and-halt
     ;; is-inf?
     is-terminated?
     ;; (go init-state)
     ;;(take 7)
     ;; (is-terminated? 8)
     )

(->> terminating-program
     (run-tick {:PC 8 :A 8}))

(let [program (->> F load-code)
      last (->> program count dec)]
  (->> program
       go-and-halt
       is-inf?))


(->> F
     load-code
     go-and-halt
     )

(defn part1 [filename]
  (->> filename
       load-code
       go-and-halt
       second
       :A))

(part1 F)

(defn correct-code [program i]
  (let [[ins op] (nth program i)
        cor (if (= ins :nop) :jmp :nop)]
    (assoc program i [cor op])))

(def terminating-program
  [[:nop 0]
   [:acc 1]
   [:jmp 4]
   [:acc 3]
   [:jmp -3]
   [:acc -99]
   [:acc 1]
   [:acc 6]])

(correct-code terminating-program 0)

(def inf-program
  [[:nop 0]
   [:acc 1]
   [:jmp 4]
   [:acc 3]
   [:jmp -3]
   [:acc -99]
   [:acc 1]
   [:jmp -4]
   [:acc 6]])


(defn part2 [filename]
  (let [program (load-code filename)]
    (->> program
         (map vector (iterate inc 0))
         (filter (fn [[_ [ins ops]]]
                   (and (not (and (= ins :nop) (= ops 0)))
                        (or (= ins :nop) (= ins :jmp)))
                   ))
         (map (fn [[i _]] i))
         (drop-while (fn [i] (->> (correct-code program i)
                                  go-and-halt
                                  is-inf?)))
         first
         (correct-code program)
         go-and-halt
         second
         :A
         )))
