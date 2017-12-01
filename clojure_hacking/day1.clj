(def s
	(->> (slurp "/home/vincent/CS/advent_of_code_2017/d1_input.txt")
		 (map #(Character/getNumericValue %))
		 (filter (partial not= -1))
		 (into [])))

(let [len (count s)	  
	  opposite-idx-1 #(mod (inc %) len)
	  m (map-indexed #(if (= %2 (s (opposite-idx-1 %1))) %2 0) s)
	  opposite-idx-2 #(mod (+ % (/ len 2)) len)
	  m2 (map-indexed #(if (= %2 (s (opposite-idx-2 %1))) %2 0) s)]

	  (println (reduce + m))
	  (println (reduce + m2)))