use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::iter::FromIterator;

fn solve() {
    let packet = "pfptpztzfznzzznszzfgzgqgbgzgmzggwlwnlngnddjttzwtwmttlrrlqqzczpzhppmjmnjnfjnjjjprpfpnfpnpznzbbnwbbdsbswwwwszzzcmcttbftffczctztffppcvccbdbhbtbcbhbrbnrbrzbztbblvlvqvcqvcvcrcprrmffhfwfjwjrjtttvqqsttcwtthwthhqssbbqhhcbhbhqqwsqspqspprbrsrmmvhvvlsvspvpddbvdbdhhgshggfbgbrggzszsnshsvhsvhhnfnhfnhhbzzthtstdssdsrdsrdrtddsrdrcrgcgdcggdbbtgbgzbzwwnlngggspggspspjjfqflqqttjbbnffszfzjjlwjwhjwjccpllzztrzttdpdbbcnntwnnctnnrrgbrbvbccvzvlvnllvrvnnvmmtrmrddjdqdrqrmmvssgllsjsrrwggvcccgwwgddzldzldzdttqrrtggdngdgfdgfgzztdtthrttszzgjglldzdhdphppjfjfzfpfsflfggpwwwjdjssqdqmdmdpmmvbvfvpfpfwfpwfppbgbwgbbpmpwpzplpjpttnwnlnppcvchhmwwmllwppwspwspswpphhdhfhccwpcplpzzjwjssfrsssgnglnglnglngncgcvvpddrcrjrssznnjllslrlmllglpggwhhllzqzssvtsvvbpphpthphdhphnhdhdqdbbmcmqcmcmddtwtqqsbbzddtzdzrzmmfccdttlvljlffzczmzrrtzzrbrllfnfmfnnrcnnctcnnncbcrbccwcnwcnnrbnrrpnprrtwwvcwchwwcbwwwnpnvvgtvvqmvvhttrnnvjjtwtfwtwrrbpbmbmzmpphwphwwqwgwlwclwccjvccsllhhrtrstthvvfjjcvcdcbdcbczcttjbttwtvvvzjzgzwgwjjsnnvrrlbrrvnvdvhvchvvglvvpssdbsszddsqsspvpbpjjgghvvlwvlvggpjjmcjmmmhjmmrgrsgrsggchhcmmqsmsttlslqqmjjbpbrrcnrnbbvqvsvtvftvtmmqzzsnscsffnrnmrmnnszsllsrrrzrszsbbchccsrsmshhzrhrrdlrrjfmlrfhvqqvmpbrntgcqqsqvjmtctflbffddfbsjvzsfdwblprszhfvltwtcfsbdlwjgsmlcrvgstjqtrtnqzbmrmgqnscqjdfnbppcdgcsstwdmdvphsqmrfmzwntjgjjvcdgbhfjqlzglgjdsdlhwwrmfqcfsvhwwfmvprpnmjppvwzjwmddtndspzqjqrpbpnrjfwqfvbtqrgngcbjvhnfbtslcpppbsfhbcmwgpccftwhnbvdrzqdtwnrtjcdlnlmhvlzvljwrzgtfjrpgzjvggtpsvcdgtsvhdzvtfbwmnptfmllgcvfmmgvpbrgnhcnpwltqmjmltsbpzmfrttsmjqwhncvtqrsmcpsnrqzmwftbltllbhzhdfzmfgbvdtgwpvngsffjmwhfhmccfrjgqcqngzlnqvsgrcdzbsmjbmflwvhjldlrdvjrmgvjvpcczdhczpbtwphvhqmhcnljbwnzqwmbctffmctlcmhzcnvprdhtzdvgbhlnnjqzcwcsrgzjjlszssnplwqjlczvftmnbnmdpbjnctnslhgsjswqjwvprdstvbstlnnwwgvsffwmprjrlfccmtgvqbghvhcngwwtzwbwcdmrfstwhtfghgvzbfgtwjglcllwrhgdzptvrrbdhbscjhmtswshjmrsbpzstwmhrwwwncbbmjnjjlzrpdrzfvstbltszvlhcqbcpgbwtzzslsrljmhmtlcvzdbszvnjhrswrjrmfpsfpplwlrsnrpnjngmhwpwqcmtslhbmlsmjhcgmzznftmhvtmzlvmcwnbqtcntqghrqcsztsgzrnmrlvrnhtpmstdflpztmwltvgppttfwhhzzgrffjchswhbljvcjwvvnqnvdvjpsclhwsrtczvjmtcsnwvnwtdllphmrthddfvcjwvggqltmhglllmqzjsbjwgdqwzzmjnrmpbqplgzjgzcdqmtsntprdwwjwthcbsghqqspszndgqdmlzdlzwfcghtbhcqpbpmnfgqzmhtnttvjttvzhllsjvmmcmmppcgssjhnzqwpzdbtmrzsfbvgmgbtbwjrzvdlmgjpzltfmcclpltsszpqbllrwbwsnbhhvfwphrcpdvbjhgmgpphrdpcmjvfsjzrqldlqthwsztzcgttdnzcsbnszcsvmcspddlmwjttggdmlpqrdrfmwfzpdbnrwtmwssvbwtmzhndmhzwtlgdwpbrzghmlbszswqlpzldbvswjgtvjvmtjwdggfsbggbwhpwjdmflhmsgtbzrtbvlpqqmpcrbhflnfmwwsvdsgnnznfrqhqgqfgdfzcdqrtdftsntpbcclhncqjjwvszmssswnscwjlpfvdvltgcmqqttnfvbptbbmlrvrwwfbwwbvlrdrfmscqwdvdjgdrghwfjsttvwngzttzzsmzqnvzdfsvrbrcwtmmjdvnzjzdsnzgtszzcwdphnjmspmdsrqwgdwlzrgghcchpbltmwnjrbhqhzdqqmbrpggjjwnqfnnsqsfzbwqjsfprvrvfwbqvhgpjvqzplnhtqszqrtsvtbptfvzmvjhshbtmqbmqrrwplzphvdvhttlmftdwltqssstzlvnslzhnmjdlsbdprbgpjvcdtcfchzqqqcnngbrmntjwfbvcdcfgbcpnvcbbcvhqfzpsmgbcvrqvjlqlqnvvzdgphfpgtrpbbwztvqjgdpnpwbffdgqzmqvgblwzmdrhwdprhcqppcggrldhcdztnhspclfcwttnqslnzvvshcwgfztvscvztdrprvnlmfsgcpfdmfjgblnhrbsmjrjdzjwvwmlllvscsvfqvhdsdljrqphcvvtcttbwvnwwzwshdcfdqnjszltmddzjgmqgvpjzpzssrmfsrgjhvqtlhsfnndnqhpbcnltmdvlhfwqcmwnqbhsfqqwnfnnfjjbsqmcdrrvlfztdprnmjfhlvcdbjtczbrpljmpcwvchdwrqbwggjnrlhcdgzfwjzzjgfnbpwbpvswqdpcrthwfcffgztrjqntczfcbsrrtrjrwgbbgjshtzvjjlqqtsbgmpsttqjqwgmmbzhshqvvrcgbdsqmtqlrgjbnvbrpzdrgqnzstfdcvdnjhcnjblmsqtfvstlptgrczhbgpllpqwfdmthgjlltmlnltzpvjvjfgtrzslsptlfplgrgpjsbhbbbwlljfdjnhqcndlprfbwpvddndpnqwqccgbqmwlrffpzjpqclwcrgjgljwzpppwltcwdqdchghfnwbhrjndjsvlqnnmlrjfgfpnvgmlhbgnhnztpjzdmltfmjtzclsbspvhfngtjmzwrwmprdfplzzwfrdnbmbgvjlczcdvmpfmtqmzjrpfhjwwzmtnzmptwnhtlbndcpshqrqqrpccqpnvnqqdprvccmdmrsbptdhrhlpcptgfsfwphfpvbcrlnbrtgwcpgjclhhvpjhcwcgghlzbmpbswgtzqhmlwdfrrdfvnbhlqhvhnfjfndlqgrvhwnnnccvgdfqtwlmbwcpdtgscfpvmbdtcdmmgqrfjvnhngqsdtzhlbjwrrcrjfswwrgbhznlwhcjlsfprbqqcqmbdjhgjmmtqmjpldgqvptqcwjmlrjtjwdfbbvhpsnmfvdwnrntqzhfgfmrtgwgddpqvvdjqvrdwdwrsbjlbrmrjjbbpjpqgsjdzfjcrsnbmtmrstcrztzhgswgghwbfltdsvrcqvvjtmjwznnnwtsmshvbbpzwltrjpmbgsbqwphmwlhgpltsgjmgbdfrlhcbfjnvpvdwzccgdhswtgplcqnsjdwfbhbbpssvfrjbzmcphzjdncjgsvrcrplhqpnwdgfvrjqgfshdwrqjdvjmggtnnghqrccgddnzndcgpgpghtvrpwftfpttvgwqqcjbvnmqzlshdrdj"
        .chars()
        .collect::<Vec<char>>();
    find_start(&packet, 4);
    find_start(&packet, 14)
}

fn find_start(packet: &Vec<char>, size: usize) {
    let mut cnt = 0;
    let mut window: HashSet<char>;
    for marker in packet.windows(size) {
        window = HashSet::from_iter(marker.iter().map(|l| l.to_owned()).clone());
        if window.len() == size {
            break;
        }
        cnt += 1;
    }
    println!("{:?}", cnt + size);
}

fn main() {
    solve();
}
