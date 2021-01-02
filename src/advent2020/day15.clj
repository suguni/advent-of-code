(ns advent2020.day15)

(def numbers [0 3 6])

(defn turn [acc]
  (let [spoken (last acc)
        turn (- (count acc) 1)
        last (.lastIndexOf (pop acc) spoken)]
    (if (= last -1)
      (conj acc 0)
      (conj acc (- turn last)))))


(->> numbers
     turn
     turn
     turn
     turn
     turn
     turn
     turn)
;; = [0 3 6 0 3 3 1 0 4 0]

;; (last
;;  (loop [acc [3 1 2]
;;         c 0]
;;    (if (= c (- 20200 3))
;;      acc
;;      (recur (turn acc)
;;             (inc c)))))

(comment
  (->> [3 2 1]
       (iterate turn)
       (drop-while #(< (count %) 20200))
       first
       last))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(defn update-turn [acc speak turn]
  (-> acc
      (update speak (fn [[_ j]] [j turn]))
      (assoc :last speak :turn (inc turn))))

;; (def init-turn {0 [-1 0]
;;                 3 [-1 1]
;;                 6 [-1 2]
;;                 :last 6
;;                 :turn 3})

;; (update-turn init-turn 0 3)

(defn turn-map [acc]
  (let [spoken (:last acc)
        turn (:turn acc)
        [i j] (get acc spoken [-1 -1])
        speak (if (< i 0) 0 (- j i))]
    (-> acc
        (update speak (fn [v]
                        (if-let [[_ j] v]
                          [j turn]
                          [-1 turn])))
        (assoc :last speak
               :turn (inc turn)))))

;; (->> init-turn
;;      turn-map
;;      turn-map
;;      turn-map
;;      turn-map
;;      turn-map
;;      turn-map
;;      turn-map)

(defn make-turn [init]
  (assoc
   (->> init
        (map (fn[i v] (vector v [-1 i]))
             (range (count init)))
        (apply concat)
        (apply hash-map))
   :last (last init)
   :turn (count init)))

;; solution!!!
(comment
  (def result
    (->> [1 12 0 20 8 16]
         make-turn
         (iterate turn-map)
         (drop-while #(< (:turn %) 30000000))
         first
         :last))

  (println result))




;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(defn alast-index-of [^longs ary
                      ^long start
                      ^long value]
  (loop [i start]
    (if (< i 0)
      -1
      (if (= (aget ary i) value)
        i
        (recur (dec i))))))

;; (alast-index-of (to-array (range 10)) 9 4)

(defn array-turn [^longs ary
                  ^long turn]
  (let [spoken (aget ary (- turn 1))
        last (alast-index-of ary (- turn 2) spoken)]
    (if (= last -1)
      (aset ary turn 0)
      (aset ary turn (- turn last 1)))))

(defn play [numbers n]
  (let [c (count numbers)
        pad (- n c)
        ary (long-array (concat numbers (repeat pad 0)))]
    (dotimes [i pad]
      (when (zero? (mod i 100000))
        (println (str "running " i)))
      (array-turn ary (+ i c)))
    (seq ary)))


;; (def result (last (play [1 12 0 20 8 16] 30000000)))
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
