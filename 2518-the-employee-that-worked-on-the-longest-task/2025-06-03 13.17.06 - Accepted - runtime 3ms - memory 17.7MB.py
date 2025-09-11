class Solution:
    def hardestWorker(self, n: int, logs: List[List[int]]) -> int:
        max_time = logs[0][1]
        result_id = logs[0][0]
        
        for i in range(1, len(logs)):
            curr_id, curr_time = logs[i]
            prev_time = logs[i - 1][1]
            duration = curr_time - prev_time

            if duration > max_time or (duration == max_time and curr_id < result_id):
                max_time = duration
                result_id = curr_id
                
        return result_id
        