import { Mapper } from './Mapper.js';

{
    var mapper = new Mapper(self);

    self.onmessage = (evt) => {
        if (evt.data instanceof Object) {
            if (evt.data.ref) {
                mapper.set_ref(evt.data.file);

            } else if (evt.data.map) {
                mapper.map(evt.data.filename, evt.data.revReads);

            } else {
                throw "Event " + JSON.stringify(evt.data) + " is not supported";
            }
        }
    }

}

