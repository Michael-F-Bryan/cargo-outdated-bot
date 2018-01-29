(function() {var implementors = {};
implementors["slog"] = [];
implementors["slog_async"] = ["impl <a class=\"trait\" href=\"slog/trait.Drain.html\" title=\"trait slog::Drain\">Drain</a> for <a class=\"struct\" href=\"slog_async/struct.AsyncCore.html\" title=\"struct slog_async::AsyncCore\">AsyncCore</a>","impl <a class=\"trait\" href=\"slog/trait.Drain.html\" title=\"trait slog::Drain\">Drain</a> for <a class=\"struct\" href=\"slog_async/struct.Async.html\" title=\"struct slog_async::Async\">Async</a>",];
implementors["slog_json"] = ["impl&lt;W&gt; <a class=\"trait\" href=\"slog/trait.Drain.html\" title=\"trait slog::Drain\">Drain</a> for <a class=\"struct\" href=\"slog_json/struct.Json.html\" title=\"struct slog_json::Json\">Json</a>&lt;W&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,&nbsp;</span>",];
implementors["slog_term"] = ["impl&lt;D&gt; <a class=\"trait\" href=\"slog/trait.Drain.html\" title=\"trait slog::Drain\">Drain</a> for <a class=\"struct\" href=\"slog_term/struct.FullFormat.html\" title=\"struct slog_term::FullFormat\">FullFormat</a>&lt;D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"slog_term/trait.Decorator.html\" title=\"trait slog_term::Decorator\">Decorator</a>,&nbsp;</span>","impl&lt;D&gt; <a class=\"trait\" href=\"slog/trait.Drain.html\" title=\"trait slog::Drain\">Drain</a> for <a class=\"struct\" href=\"slog_term/struct.CompactFormat.html\" title=\"struct slog_term::CompactFormat\">CompactFormat</a>&lt;D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"slog_term/trait.Decorator.html\" title=\"trait slog_term::Decorator\">Decorator</a>,&nbsp;</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
