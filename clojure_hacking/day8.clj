(def lines (clojure.string/split-lines (slurp "/home/vincent/CS/advent_of_code_2017/d8_input.txt")))

(defn- process-line [[regs max-seen :as old-state] line]
       (let [splits (clojure.string/split line #"\s+")
             target (splits 0)
             val (* (if (= (splits 1) "dec") -1 1) (Integer/parseInt (splits 2)))
             pred-matches (let [pred-var (get regs (splits 4) 0)
                                pred-val (Integer/parseInt (splits 6))
                                pred (case (splits 5)
                                           ">" > "<" <
                                           ">=" >= "<=" <=
                                           "==" = "!=" not=)]
                               (pred pred-var pred-val))
             new-val (+ (get regs target 0) val)]
            (if pred-matches
              [(assoc regs target new-val) (max max-seen new-val)]
              old-state)))

(let [[regs max-seen] (reduce process-line [{} 0] lines)]
     (println "part 1:" (apply max (vals regs)))
     (println "part 2:" max-seen))
