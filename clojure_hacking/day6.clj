(def input [14 0 15 12 11 11 3 5 1 6 8 4 9 1 8 4])

; these two could probably be more functional, but I want to do it in one pass :P
(defn- find-hi [nums]
       (loop [num-max Long/MIN_VALUE
              max-idx 0
              cur-idx 0]
             (cond
               (>= cur-idx (count nums)) max-idx
               (> (nums cur-idx) num-max) (recur (nums cur-idx) cur-idx (inc cur-idx))
               :else (recur num-max max-idx (inc cur-idx)))))

(defn- redistribute [nums idx]
       (loop [res (assoc! (transient nums) idx 0)
              redist (nums idx)
              cur-idx (mod (inc idx) (count nums))]
             (if (zero? redist)
               (persistent! res)
               (recur (assoc! res cur-idx (inc (res cur-idx)))
                      (dec redist)
                      (mod (inc cur-idx) (count nums))))))

(loop [seen? {input 0}
       cur input
       iters 0]
      (let [hi (find-hi cur)
            rd (redistribute cur hi)]
           (if (seen? rd)
             (do (println "part 1:" (inc iters))
                 (println "part 2:" (- (inc iters) (seen? rd))))
             (recur (assoc seen? rd (inc iters)) rd (inc iters)))))
