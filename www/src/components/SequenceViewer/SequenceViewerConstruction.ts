export class Row {
    sequence: string[];
    mapped_sequence: string[][];
    font_widths: number[];
    nucleotides_numbers: number[];

    constructor(sequence: string[], mapped_sequence: string[][], font_widths: number[], nucleotides_numbers: number[]) {
        this.sequence = sequence;
        this.mapped_sequence = mapped_sequence;
        this.font_widths = font_widths;
        this.nucleotides_numbers = nucleotides_numbers;
    }
}

export function Rows(font_size: Number, font_family: string, whole_mapped_sequences_chrom: string[][], whole_sequences: string[]) {
    const nb_mapping = whole_mapped_sequences_chrom[0].length;

    const width_page = document.body.clientWidth - 20;

    let sequence: string[];
    let mapped_sequences: string[][];
    let font_widths: number[];
    let nucleotides_numbers: number[];
    const font_width = getTextWidth(font_size, font_family);
    
    const Rows = [];
    let current_width = 0;

    let nucleotide = "";
    const nucleotide_mapped: string[] = [];
    let skip = false;

    for (let seq_i = 0; seq_i < whole_sequences.length; seq_i++){
        sequence = [];
        mapped_sequences = [];
        for (let _ = 0; _ < nb_mapping; _++){
            mapped_sequences.push([]);
        }
        font_widths = [];
        nucleotides_numbers = [];

        current_width = 0;

        const whole_sequence = whole_sequences[seq_i];
        const whole_mapped_sequences: string[] = whole_mapped_sequences_chrom[seq_i];

        for (let i = 0; i < whole_sequence.length; i++) {
            if (current_width + font_width > width_page - 40) {
                current_width = 0;
                Rows.push(new Row(
                    sequence, 
                    mapped_sequences, 
                    font_widths, 
                    nucleotides_numbers
                ));
                sequence = [];
                mapped_sequences = [];
                for (let _ = 0; _ < nb_mapping; _++){
                    mapped_sequences.push([]);
                }
                font_widths = [];
                nucleotides_numbers = [];
            }

            nucleotide = whole_sequence[i];

            if (whole_mapped_sequences.every(mapped_sequence => mapped_sequence[i] == "-")) {
                if (!skip) {

                    if (current_width + font_width > width_page - 40) {
                        current_width = 0;
                        Rows.push(new Row(
                            sequence, 
                            mapped_sequences, 
                            font_widths, 
                            nucleotides_numbers
                        ));
                        sequence = [];
                        mapped_sequences = [];
                        for (let _ = 0; _ < nb_mapping; _++){
                            mapped_sequences.push([]);
                        }
                        font_widths = [];
                        nucleotides_numbers = [];
                    }
                    
                    nucleotide = ".......";
                    for (let j = 0; j < nb_mapping; j++){
                        nucleotide_mapped[j] = ".......";
                    }
                    const font_width_skip = getTextWidth(font_size, font_family, nucleotide_mapped[0]) + 2;
                    current_width += font_width_skip;

                    font_widths.push(font_width_skip);
                    nucleotides_numbers.push(1);
                    sequence.push(nucleotide);
                    for (let j = 0; j < nb_mapping; j++){
                        mapped_sequences[j].push(nucleotide_mapped[j]);
                    }
                }
                skip = true;
                continue;
            }
            else {
                for (let j = 0; j < nb_mapping; j++){

                    if (whole_mapped_sequences[j][i] == nucleotide) {
                        nucleotide_mapped[j] = "-";
                    }
                    else if (whole_mapped_sequences[j][i] == "-") {
                        nucleotide_mapped[j] = " ";
                    }
                    else {
                        nucleotide_mapped[j] = whole_mapped_sequences[j][i];
                    }
                }
                skip = false;
            }

            current_width += font_width;
            font_widths.push(font_width);
            nucleotides_numbers.push(i + 1);
            sequence.push(nucleotide);
            for (let j = 0; j < nb_mapping; j++){
                mapped_sequences[j].push(nucleotide_mapped[j]);
            }
        }
        Rows.push(new Row(
            sequence,
            mapped_sequences,
            font_widths,
            nucleotides_numbers
        ));
    }

    return Rows;
}

function getTextWidth(font_size: Number, font_family: string, text = "G") {  
    const inputText = text; 
    const font = font_size + "px " + font_family; 
    
    const canvas = document.createElement("canvas"); 
    const context = canvas.getContext("2d"); 
    if (context) {
        context.font = font; 
    }
    else {
        return 0;
    }
    
    const width = context?.measureText(inputText).width; 
    return width;
}