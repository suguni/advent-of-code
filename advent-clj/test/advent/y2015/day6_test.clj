(ns advent.y2015.day6-test
  (:require [clojure.test :refer :all]
            [advent.y2015.day6 :as s]))

(deftest parse
  (is (= (s/parse "turn on 0,0 through 999,999") {:turn :on :s [0 0] :e [999 999]}))
  (is (= (s/parse "toggle 0,0 through 999,0") {:turn :toggle :s [0 0] :e [999 0]}))
  (is (= (s/parse "turn off 499,499 through 500,500") {:turn :off :s [499 499] :e [500 500]})))

(defn create-grid [size t]
  (int-array (* size size) t))

(deftest turn
  (is (= (vec (s/turn
                (create-grid 10 0) 10
                {:turn :on :s [0 0] :e [9 9]}
                s/control1))
         (vec (create-grid 10 1))))
  (is (= (vec (s/turn
                (create-grid 10 0) 10
                {:turn :toggle :s [0 0] :e [9 9]}
                s/control1))
         (vec (create-grid 10 1))))
  (is (= (vec (s/turn
                (create-grid 3 0) 3
                {:turn :toggle :s [0 0] :e [1 1]}
                s/control1))
         [1 1 0 1 1 0 0 0 0])))

#_(deftest solve-line
  (is (= (s/solve-line "turn on 0,0 through 999,999") 0)))