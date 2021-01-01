(ns advent.day21
  (:require [clojure.string :as str]
            [clojure.set :as set]))

(defn parse-food [line]
  (let [[ingredients _ allegrens]
        (->> line
             (re-seq #"\w+")
             (partition-by #(= "contains" %)))]
    [(apply hash-set ingredients) (apply hash-set allegrens)]))

(parse-food "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)")

(def foodtext "
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
")

(defn parse-foods [foods]
  (->> foods
       str/trim
       str/split-lines
       (map parse-food)))

(parse-foods foodtext)

(defn find-candidates [foods]
  (let [ingredients (->> foods
                         (map first)
                         (reduce into #{}))
        allegrens (->> foods
                       (map second)
                       (reduce into #{}))]
    (->> allegrens
         (map (fn [allegren]
                [allegren
                 (->> foods
                      (filter (fn [[_ ale]] (ale allegren)))
                      (map first)
                      (reduce set/intersection ingredients))])))))

(defn sort-by-ingredient-count [candidates]
  (sort-by (comp count second) candidates))

(->> foodtext
     parse-foods)

(defn allegren-count-contains-ingredient [candidates ingredient]
  (->> candidates
       (filter #((second %) ingredient))
       count))

(allegren-count-contains-ingredient
 [["dairy" #{"mxmxvkd"}]
  ["soy" #{"sqjhc" "fvjkl"}]
  ["fish" #{"sqjhc" "mxmxvkd"}]]
 "sqjhc")

(defn remove-ingredient [candidates ingredient]
  (->> candidates
       (map (fn [[ale ings]]
              [ale (disj ings ingredient)]))))

(disj #{:a :b} :a)

(remove-ingredient
 [["dairy" #{"mxmxvkd"}]
  ["soy" #{"sqjhc" "fvjkl"}]
  ["fish" #{"sqjhc" "mxmxvkd"}]]
 "sqjhc")


(defn match-allegren-ingredient [candidates]
  (loop [candidates candidates
         result []]
    (if (empty? candidates)
      result
      (let [[ale ings] (first candidates)
            ;; ings 가 empty 면 ERROR!
            my-ing (->> ings
                        (map (fn [ing]
                               [ing (allegren-count-contains-ingredient candidates ing)]))
                        (sort-by second)
                        first
                        first)
            r (conj result [ale my-ing])
            c (remove-ingredient (rest candidates) my-ing)]
        (recur c r)))))


(defn solve-part1 [foodtext]
  (let [food-table (->> foodtext
                        parse-foods)

        all-ingredients (->> food-table
                             (map first)
                             (reduce into #{}))

        matching-ingredients (->> food-table
                                  find-candidates
                                  sort-by-ingredient-count
                                  match-allegren-ingredient
                                  (map second)
                                  (into #{}))
        not-ingreds (set/difference all-ingredients matching-ingredients)]
    (->> food-table
         (map first)
         (map #(set/intersection not-ingreds %))
         (map count)
         (apply +))))


(->> (slurp "resources/day21-input.txt")
     str/trim
     solve-part1)


(defn solve-part2 [foodtext]
  (let [food-table (->> foodtext
                        parse-foods)
        matching-ingredients (->> food-table
                                  find-candidates
                                  sort-by-ingredient-count
                                  match-allegren-ingredient
                                  (sort-by first))]
    (->> matching-ingredients
         (map second)
         (str/join ","))))

(->> (slurp "resources/day21-input.txt")
     str/trim
     solve-part2)
"prxmdlz,ncjv,knprxg,lxjtns,vzzz,clg,cxfz,qdfpq"
