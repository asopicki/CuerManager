export class Cuecard {
    id: number;
	uuid: String;
	phase: String;
	rhythm: String;
	title: String;
	steplevel: String;
	difficulty: String;
	choreographer: String;
	meta: Object;
	content: String;
	karaoke_marks: String;
	music_file: String;

    constructor(data: any) {
        Object.assign(this, data);
        this.meta = JSON.parse(data.meta);
	}
	
	getLink(): String{
		return '/cuecard/' + this.uuid;
	}

	asTarget(): String {
		let result = this.title;

		if (result) {
			let trans = {
				'&.+?;': '',
				'[^\\w\\d _-]': '',
				'\\s+': '-',
				'(-)+': '-'
			};
	
			for (let [key, value] of Object.entries(trans)) {
				let r = new RegExp(key, 'ig');
	
				result = result.replace(r,value);
			}

			
			return result.toLowerCase();
		}
		

		return this.uuid;
	}
}
