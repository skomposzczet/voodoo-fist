class RandomProposition {
    readonly texts = [
        'Perish men',
        'Crumble cities',
        'Sarcrifice child',
        'Write bad unit tests',
        'Mess with production code',
        'Kill some linux proccesses',
        'Cook meth with Mr. White',
        'Sit on Adriana\'s dog',
        'Arrange The Red Wedding',
        'Trap Rita Skeeter in jar',
        'Defeat Adam Smasher',
        'Argue with Johny Silverhand',
        'git pull --sum bitches',
    ];

    get() {
        return this.texts[Math.floor(Math.random() * this.texts.length)];
    }
}

export default new RandomProposition();