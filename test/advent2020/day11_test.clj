(ns advent2020.day11-test
  (:require [advent2020.day11 :as s]
            [clojure.test :refer [deftest is]]))

(def layout1 (s/create-layout 3 3 (repeat 9 :L)))
(def layout2 (s/create-layout 3 3 (repeat 9 :#)))
(def layout3 (s/create-layout 3 3 [:L :. :L :L :L :L :L :. :L]))

(comment
  (deftest adj
    (is (= (s/adjacency layout1 [1 1])
           [:L :L :L :L :L :L :L :L]))
    (is (= (s/adjacency layout1 [0 0])
           [:L :L :L]))
    (is (= (s/adjacency layout1 [2 2])
           [:L :L :L]))
    ))

(comment
  (deftest update-round
    (is (= (s/get-cell layout3 [2 1]) :.))
    (is (= (s/get-cell layout1 [0 0]) :L))
    (is (= (s/update-cell layout1 [1 1]) :#))
    (is (= (s/update-cell layout2 [1 1]) :L))

    ;(is (= (s/update-cell s/input11-layout [0 0]) :#))
    ;(is (= (s/update-cell s/input11-layout [0 1]) :.))
    ;(is (= (s/update-cell s/input11-layout [1 1]) :#))
    ;(is (= (s/update-cell s/input11-layout [1 2]) :#))

    (is (s/layout-changed? (s/create-layout 1 2 [:L :L])
                           (s/create-layout 1 2 [:L :E])))

    (is (not (s/layout-changed? (s/create-layout 1 2 [:L :L])
                                (s/create-layout 1 2 [:L :L]))))

    (is (= (s/update-layout layout1) layout2))
    (is (= (s/update-cell (s/create-layout 3 3 [:L :. :L
                                                :L :L :L
                                                :L :. :L]) [0 1])
           :.))
    (is (= (s/update-layout (s/create-layout 3 3 [:L :. :L
                                                  :L :L :L
                                                  :L :. :L]))
           (s/create-layout 3 3 [:# :. :# :# :# :# :# :. :#])))

    ;(is (= (s/update-layout s/input11-layout) s/input12-layout))
    ;(is (= (s/update-layout s/input12-layout) s/input13-layout))
    ))
