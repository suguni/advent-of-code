(ns advent2020.day8-test
  (:require [advent2020.day8 :as sut]
            [clojure.test :as t]))

(t/deftest machine-run
  "Day 8 Test"
  (t/is (= {:PC 1 :A 0}
           (sut/run sut/init-state :nop 0)))
  (t/is (= {:PC 1 :A 2}
           (sut/run sut/init-state :acc 2)))
  (t/is (= {:PC 6 :A 0}
           (sut/run {:PC 2 :A 0} :jmp 4))))

(t/deftest machine-run-tick
  "Day 8 Machine run tick"
  (t/is (= {:PC 1 :A 0}
           (sut/run-tick sut/init-state sut/program )))
  (t/is (= {:PC 2 :A 1}
           (sut/run-tick {:PC 1 :A 0} sut/program ))))


(t/deftest machine-go
  "Day 8 Machine go"
  (t/is (= {:PC 1 :A 0}
           (nth (sut/go sut/init-state sut/program) 1)))
  (t/is (= {:PC 2 :A 1}
           (nth (sut/go sut/init-state sut/program) 2))))
