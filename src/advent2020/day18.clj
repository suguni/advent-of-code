(ns advent2020.day18
  (:require [clojure.string :as str]))

(def input
  (->> (slurp "resources/2020/day18-input.txt")
       str/trim
       str/split-lines))

(def samples (->> "
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
"
                  str/trim
                  str/split-lines))


(def numerics (set "0123456789"))
(def operators (set "*+"))
(def open-paren (set "("))
(def close-paren (set ")"))

(defn build-tree [tree cs]
  (if (zero? (.length cs))
    [tree ""]
    (let [c (.charAt cs 0)
          r (.substring cs 1)]
      (cond
        (= \space c) (recur tree r)
        (numerics c) (let [p (last tree)
                           n (Character/getNumericValue c)
                           tree (if (number? p)
                                  (conj (pop tree) (+ (* p 10) n))
                                  (conj tree n))]
                       (recur tree r))
        (operators c) (recur (conj tree c) r)
        (open-paren c) (let [[sub-tree chars] (build-tree [] r)]
                         (recur (conj tree sub-tree) chars))
        (close-paren c) [tree r]))))

(build-tree [] (first samples))

(defn eval-tree [stack tree]
  (if (seq tree)
    (let [v (first tree)
          r (rest tree)]
      (cond
        (number? v) (if (empty? stack)
                      (eval-tree (conj stack v) r)
                      (let [op (last stack)
                            w (last (pop stack))
                            stack (pop (pop stack))
                            result (case op
                                     \* (* w v)
                                     \+ (+ w v))]
                        (eval-tree (conj stack result) r)))
        (vector? v) (let [sub-result (eval-tree [] v)]
                      (eval-tree stack (concat sub-result r)))
        (operators v) (eval-tree (conj stack v) r)))
    stack))

(defn eval-tree2 [stack tree]
  (if (seq tree)
    (let [v (first tree)
          r (rest tree)]
      (cond
        (number? v) (if (empty? stack)
                      (recur (conj stack v) r)
                      (let [op (last stack)]
                        (if (= op \+)
                          (let [w (last (pop stack))
                                stack (pop (pop stack))
                                result (+ w v)]
                            (recur (conj stack result) r))
                          (recur (conj stack v) r))))
        (vector? v) (let [sub-result (eval-tree2 [] v)]
                      (recur stack (concat sub-result r)))
        (operators v) (recur (conj stack v) r)))
    [(reduce (fn [acc v] (if (number? v) (* acc v) acc)) 1 stack)]))

(eval-tree2 [] [[2 \* 1 \+ 3]])

(eval-tree2 [] [2 \+ 3])

(eval-tree2 [] [2 \* 3 \+ [4 \* 5]])

(->> input
     (map #(build-tree [] %))
     (map #(->> %
                first
                (eval-tree2 [])
                first))
     (apply +)
     )
 ;; 362880372308125

(->> "1 + 2 * 3 + 4 * 5 + 6"
     (build-tree [])
     first
     (eval-tree [])
     first)
71
(->> "1 + (2 * 3) + (4 * (5 + 6))"
     (build-tree [])
     first
     (eval-tree [])
     first)
51
