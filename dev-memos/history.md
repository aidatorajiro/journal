Journal History

[TODO]
❌一旦諦め TODO: Looking for typesetting and hopefully COLR support...
    oh! that looks nice!  https://github.com/dfrg/swash
❌ TODO: 分解やタグ付けなどの、データを整える作業を一種のゲームとして楽しむ、みたいな...
❌ TODO: Making 3D atmosphere?
❌ TODO: id 付番 or 表示 => mozc dict をベースにする
    無機的なIDではなく、さまざまな文脈と繋がることのできるID。一方でuniquenessなどの用件も携えている
❌ TODO: タグづけゲーム
🟢 TODO: 分岐を作るゲーム　異なるjournalのfragment同士をくっつけるpathを作る。そうして新たなjournalが誕生する。
    [そういえば、fragment自体にIDをつけなきゃね... fragment自体の分岐・合流・他のfragmentとの結合によって無数のpathができる仕組みを作りたい]
🟢 TODO: データserialize & deserialize保存メカニズム！これがないと始まらないよね...
🟢 TODO: addメモ 人力でメモを追加する
🟢 TODO: migrate facebookからデータを引っ張ってくる

[2022-09-22]
when i open VSCode i couldnt use extension...
git init
gibo dump macos rust >> .gitignore

Read https://bevy-cheatbook.github.io/input/dnd.html

[2022-09-23]
Figuring out what can be used in the framework
Playing https://ramirezmike2.itch.io/usa-football-league-scouting-combine-xlv (winner of Bevy Jam #2 : https://itch.io/jam/bevy-jam-2/results)
    finding out it's quite slow...
    returning to old days of internet?
Search documentation - https://docs.rs/bevy/latest/bevy/?search=bundle
    Looking for "bundle" to see what can be used (such as texts, buttons, etc.)
seems like Bevy doesn't have text input...
    But looking MIDI section on the unofficial cookbook, maybe there's a possibility of 3rd-party extension library? → found bevy_egui! https://github.com/mvlabat/bevy_egui
Bevy's "data-driven" way is similar to Modding! (especially that of X series, or maybe civilization or stellaris?)

[2022-09-27]
https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs
    Stage についてべんきょうしたよ
Sub-apps and multiple window
    もはやドキュメントすら書かれていない領域に突入したか...
Meaning of Equal and Labels (like x, y, z) classifies language
    haskell - definition → Free to split (pattern-match), fuse numbers and apply functions to labels (it is functional language, so "labels" aren't "variables"), like "x = (y, z)" or "(a, b) = c" or "y = f(x)". The notation "x =" is just the same as replacement of a code thereafter and even itself. Therefore, one may be able to fuse the whole program into one function without using any labels other than "main = ...".
    python - normal? language → You can freely apply functions, split or merge variables.  The notation "x =" is NOT the same as replacement of code thereafter. For example, x = [1, 2, 3]; y = func(x); z = x + [4, 5] may not result in z = [1, 2, 3, 4, 5]. The first line does not promise that you can replace "x" with "[1, 2, 3]" in the last line ("z = x + [4, 5]"). In a simpler case, "x = x + 1" just means "add 1 to x" in Python, while in Haskell, it guarantees that you can replace "x" with "x + 1" in the very sentence "x = x + 1", therefore same as "x = (...(... + 1) + 1) + 1) + 1) + 1" (fixpoint operation).
    rust - borrowing → The notation "x =" is NOT the same as replacement of code thereafter. In addition to the discussion on Python above, "=" also determines which variable is "borrow" and which variable is "move", deciding the timing these variables are freed. Therefore, you cannot freely merge or split variables through pattern-match or even applying function. Every time you apply function or pattern-match variables, you have to follow borrowing rules...

    このような　言語ごとの縛りの違いもArchitectureの一つかも？
Quite useful page! https://bevy-cheatbook.github.io/builtins.html#bundles

[2022-09-29]
やっぱりソフトウェアっていうのは、作る努力よりも使う努力の方が圧倒的に大きいというか、入力した情報によってソフトウェアは無限にさまざまな使われ方がされうるのが、ソフトウェアの「想定された使われ方」を打破することにもなる一方で、危険性でもあるよね...
ソフトウェアを作る時、どういうふうにそれが使われるかというリスクをなかなか考えづらいというのがある
(cif fileについて調べていて思ったこと...結晶構造一つとっても、アプリケーション自体は自動的にファイルを読み取って点や線を記述にしたがって描画するだけだが、そのアプリケーションが描画しうる構造には無数にさまざまなものがあって、プログラマーはそのさまざまな構造を全て把握することはできない...)

Why Rust is Rusty? → 
    There are so many "sugar syntaxes" in Rust.
    Rust's language specificaitons aren't simple, rather, some of the specificaitons seem to exist for a specific purpose...
    For example, things like "MemoField {..default()}". Where default() comes from!? → actually this is from Bevy, not Rust. Through importing Bevy, you can simplify Default:default to just default...
    And there are many different ways to do similar or same thing.
    For example, how to unpack Option? You can match, or use unpack function, or use ? notation. (probably there's other methods that I don't know...)
        If you use match, you can nest the whole block of code with match, or otherwise, you can write the match on the first line and use break, continue or return.
    Unlike other languages, becoming able to read and analyze Rust code is itself a hard problem, because there are many syntaxes you don't know!
    For Haskell, theoretically, it is enough to know several sugar syntaxes (like `div` and do) and how to define typeclass, to be able to read Haskell code, if you are keen to look the source code back and forth to analyze what it does.
    In Haskell, even symbols that look like language features (such as . $ <$> =<< << >>), are actually one of normal functions with "infix" parameters. (Though, there are many so-called "extensions" that extend syntax to do weird things...) 
    So, Rust is not a simple language. Rather, it's complex and rusty.
Difference between Cloud Service and Web Services before cloud...
    Perhaps, difference between cloud service and web services before the era of cloud is that cloud service is autonomous and running various micro-programs behind the user interface, making the platform more opaque than web services.
    Contrary, web services are running according to user's command. In most cases, user is the originator of the program, like clicking a button and send some request through HTTP(S).
    In other words, cloud service is like a simulation game. In a simulation game, numerous micro-programs are running behind the user interface and the player can only see what's happening on the surface. Also, the ways the player can interact with the game are limited.
    In many cases, the player isn't the originator of these micro-programs. Instead, they are invoked by other programs, or periodic events, or happening randomly.
    Working with bevy, I feel like writing bevy code is like making a Mod program for space-simulation games such as Stellaris (by Paradox Interactive, Sweden) or X series (by Egosoft, Germany).
    Bevy's "data-driven" mechanism is 

[2022-10-04]
Working with fragment, entry, history hierarchy, and tags.
    list of fragments makes an entry.
    the graph structure of fragments makes fragment history.
    the graph structure of entries makes entry history.
→ look for graph structure: discussion here https://github.com/bevyengine/bevy/issues/5430

[2022-10-11]

Tutorial Reading: issues with language used in programming (executed, captured, etc.)
https://www.reuters.com/article/us-amazon-com-jobs-automation-insight-idUSKCN1MK08G

[2022-10-18]
difficult thing is... Closure/function is difficult to use in Rust... because of BORROWING!!! It is easier to avoid repetition by for-each loop than using closure, and using clousure than using functions.
Difficulty: function > closure > loops

Rust mut borrowing: you have to separate code into small components, because you can't borrow mut variable twice. if you write `let mut x = abc.def(1)`, then you have to focus on programming about x. You cannot do like `let mut y = abc.def(2)`.

まあ恩恵は結構あるけどね...モジュールごとに分けることの...ある機能を変更することで、他のさまざまなコードも変えなければいけなくなることがあまり起こらない。関数を変える必要があるのは、決まってデータ構造を変えた時のみだから。データ構造を変えない限り、関数はそのままでもコンパイルは絶対に通る。

[2022-10-21]

Well... Sometimes the app crashes... It happen in about 50% probability, so it's a very weird bug. The problem occurs when a system is receiving an Event that is supposed to be emitted AFTER an "entity" is created. However, if the system tries to access the entity, it crashes. (Which means the entity isn't exist.)

Why??? maybe parallelism is related? The state of entities is not synced even after an event is emitted???

→ Configuring system order (the one that creates entity runs AFTER the one that accesses the entity)
→ You can also use exclusive_system (but it's costly therefore "crazy", according to Bevy Cookbook https://bevy-cheatbook.github.io/programming/exclusive.html)
→ Oh recent update... https://github.com/bevyengine/bevy/commit/dc3f801239d25287dc11d7cda35b00d6e3d3dbcf

Reflectionすごい！これ使えば本格的にmodding環境が整うじゃん！てかbevyってもはやmodding engineなのでは。。。

→ I first thinking of using "scene" features of bevy, but... it turns out to be difficult, because I'm using references to entity within components...
Wanted to use scene, but Vec<Entity> doesn't work well. → maybe use MapEntities? (after reading source code for Children, I discovered MapEntities, which act as a coordinator when there's a conflict between Entity IDs) → success !!!


[2022-10-27]
documentation - use mdbook, as well as rustdoc. It seemsa that these two are different documentation programs. The former is suited for describing "overview", "tutorial" or "example" things, and the latter is suited for API documentation?

[2022-10-30]
generate json schema from file → https://stackoverflow.com/questions/7341537/tool-to-generate-json-schema-from-json-data

generate rust struct from schema → https://jsontypedef.com/docs/rust-codegen/

こんなのもあった → https://crates.io/crates/json_typegen_cli

https://stackoverflow.com/questions/50008296/facebook-json-badly-encoded