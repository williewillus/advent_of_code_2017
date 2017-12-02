(def lines (with-open [rdr (clojure.java.io/reader "/home/vincent/CS/advent_of_code_2017/d2_input.txt")]
                      (->> (line-seq rdr)
                           (map (fn [line]
                                    (map #(Integer/parseInt %)
                                         (clojure.string/split line #"\s"))))
                           (into []))))

(->> lines
     (map (fn [line]
              (let [line-min (apply min line)
                    line-max (apply max line)]
                   (Math/abs (- line-max line-min)))))
     (reduce +)
     (println "part 1:"))

(defn- get-quot [line]
       ; simply pull the first one off, problem guarantees it's unique
       (first
         (for [a line
               b line
               :let [lmax (max a b)
                     lmin (min a b)]
               :when (and (not= a b)
                          (zero? (mod lmax lmin)))]
              (/ lmax lmin))))

(->> lines
     (map get-quot)
     (reduce +)
     (println "part 2:"))