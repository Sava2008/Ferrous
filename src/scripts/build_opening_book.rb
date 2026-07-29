require "open3"

STARTING_POS = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
ENGINE_SEARCH_DEPTH = 10
AVERAGE_OPENING_DEPTH = 16  # how many plies the engine will search from STARTING_POS on average, divide by 2 to get total moves
GOOD_REPLIES_PER_POSITION = 2.3
PATH_TO_ENGINE = "Ferrous_v0.5.0-dev8_lmr_fix.exe"


class SearchProgress
    class << self
        attr_accessor :approximate_total_moves, :plies_explored, :percent_finished, :current_ply
    end

    @approximate_total_moves = 0
    @plies_explored = 0
    @percent_finished = 0.0
    @current_ply = 0


    def self.calculate_total_moves
        @approximate_total_moves = (
            ENGINE_SEARCH_DEPTH * AVERAGE_OPENING_DEPTH * GOOD_REPLIES_PER_POSITION
        ).round
    end

    def self.calculate_percent_finished
        @percent_finished = (@plies_explored.to_f / @approximate_total_moves.to_f).round(2)
    end
end


class EngineCommunication
    class << self
        attr_accessor :current_position
        attr_writer :current_position
    end

    @current_position = STARTING_POS
    def self.find_move(stdin, stdout, stderr, wait_thr)
        stdin.puts "position fen #{self.current_position}"
        stdin.puts "go depth #{ENGINE_SEARCH_DEPTH}"

        best_move = ""
        line_count = 0
        stdout.each_line do |newline|
            if line_count > 500
                raise RuntimeError
            end
            if newline.start_with?("bestmove ")
                best_move = newline[9..]
            end
            line_count += 1
        end
        return best_move
    end

    def self.start_new_game(stdin, stdout, stderr, wait_thr)
        stdin.puts "ucinewgame"
    end

    def self.open_engine(path)
        Open3.popen3(path)
    end
end

def main
    SearchProgress.calculate_total_moves
    SearchProgress.calculate_percent_finished
    puts SearchProgress.percent_finished

    engine_stdin, engine_stdout, engine_stderr, engine_wait_thr = EngineCommunication.open_engine(PATH_TO_ENGINE)
    begin
        until SearchProgress.current_ply >= AVERAGE_OPENING_DEPTH
            puts EngineCommunication.find_move(engine_stdin, engine_stdout, engine_stderr, engine_wait_thr)
            SearchProgress.current_ply += 1
            # todo: 
            # - change EngineCommunication.current_position;
            # - make a Ruby chess module in Rust using magnus crate;
        end
        stdin.close
        stdout.close
        stderr.close
    rescue StandardError => e
        puts "error occurred: #{e.message}"
    ensure
        stdin.close unless stdin.closed?
        stdout.close unless stdout.closed?
        stderr.close unless stderr.closed?
    end
end

if __FILE__ == $PROGRAM_NAME
    main
end