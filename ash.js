#!/bin/env node

const sh = {};

const o = process.stdout.write.bind(process.stdout);
sh.env = {};

sh.env.PS1 = "$ ";


sh.key = key => {
	if (key === "\x03") {
		sh.stop()
		return;
	}
	
	if (sh.prompt.active) {
		if (key === "\b") {
			sh.prompt = sh.prompt.content.slice(0, sh.prompt.cur) +  - 1, sh.prompt.cur);
			sh.prompt.cur -= 1;
			if (sh.prompt.cur < 0) sh.prompt.cur = 0;
			
		}
		sh.prompt += key;
	}
	// o("\x1b[2K\x1b[1G" + sh.env.PS1 + sh.prompt); // flush
};

sh.prompt = () => {
	sh.prompt.active = true;
	sh.prompt.content = "";
	sh.prompt.cur = 0;
	o("\x1b[2K\x1b[1G" + sh.env.PS1); // flush
}
sh.cur = 0;
sh.text = 0;

sh.start = () => {
	process.stdin.setRawMode(true);
	process.stdin.setEncoding("utf8");
	process.stdin.on("data", sh.key);
	o(sh.env.PS1 || "");
};

sh.stop = () => {
	process.stdin.off("data", sh.key);
	process.stdin.setRawMode(false);
	process.exit(0);
};

sh.start();
process.once("SIGINT", sh.stop);
process.once("SIGTERM", sh.stop);