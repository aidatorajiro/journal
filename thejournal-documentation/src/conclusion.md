# Conclusion

やっぱりソフトウェアっていうのは、作る努力よりも使う努力の方が圧倒的に大きいというか、入力した情報によってソフトウェアは無限にさまざまな使われ方がされうるのが、ソフトウェアの「想定された使われ方」を打破することにもなる一方で、危険性でもあるよね...
ソフトウェアを作る時、どういうふうにそれが使われるかというリスクをなかなか考えづらいというのがある
(cif fileについて調べていて思ったこと...結晶構造一つとっても、アプリケーション自体は自動的にファイルを読み取って点や線を記述にしたがって描画するだけだが、そのアプリケーションが描画しうる構造には無数にさまざまなものがあって、プログラマーはそのさまざまな構造を全て把握することはできない...)

Difference between Cloud Service and Web Services before cloud...
    Perhaps, difference between cloud service and web services before the era of cloud is that cloud service is autonomous and running various micro-programs behind the user interface, making the platform more opaque than web services.
    Contrary, web services are running according to user's command. In most cases, user is the originator of the program, like clicking a button and send some request through HTTP(S).
    In other words, cloud service is like a simulation game. In a simulation game, numerous micro-programs are running behind the user interface and the player can only see what's happening on the surface. Also, the ways the player can interact with the game are limited.
    In many cases, the player isn't the originator of these micro-programs. Instead, they are invoked by other programs, or periodic events, or happening randomly.
    Working with bevy, I feel like writing bevy code is like making a Mod program for space-simulation games such as Stellaris (by Paradox Interactive, Sweden) or X series (by Egosoft, Germany).
    Bevy's "data-driven" mechanism is 