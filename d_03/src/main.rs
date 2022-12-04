fn main() {
    let data_str = get_data_str();
    println!("First challenge: {}", first_challenge(data_str));
    println!("Second challenge: {}", second_challenge(data_str));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_split_into_compartiment() {
        let data_str = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let compartments = split_into_compartment(data_str);
        assert_eq!(compartments, ("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
    }

    #[test]
    fn test_find_duplicate() {
        let compartments = ("vJrwpWtwJgWr", "hcsFMMfFFhFp");
        let duplicate = find_duplicate(compartments);
        assert_eq!(duplicate, 'p');
    }

    #[test]
    fn test_compute_item_priority() {
        let duplicate = 'p';
        let priority = compute_item_priority(duplicate);
        assert_eq!(priority, 16);
    }

    #[test]
    fn test_compute_rucksack_priority() {
        let data_str = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let priority = compute_rucksack_priority(data_str);
        assert_eq!(priority, 16);
    }

    #[test]
    fn test_first_challenge() {
        let data = get_sample_data_str();
        let priority = first_challenge(data);
        assert_eq!(priority, 157)
    }
    
    #[test]
    fn test_find_badge() {
        let data = ("vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg");
        let badge = find_badge(&data);
        assert_eq!(badge, 'r')
    }
    
    #[test]
    fn test_compute_group_priority() {
        let data = ("vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg");
        let priority = compute_group_priority(&data);
        assert_eq!(priority, 18)
    }
    
    #[test]
    fn test_split_into_group() {
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";
        let group = split_into_group(data);
        assert_eq!(group, [("vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg")])
    }
    
    #[test]
    fn test_second_challenge() {
        let data = get_sample_data_str();
        let priority = second_challenge(data);
        assert_eq!(priority, 70)
    }
}

fn first_challenge(data:&str) -> i32 {
    data.split("\n").map(|rucksack| compute_rucksack_priority(rucksack)).sum()
}

fn compute_rucksack_priority(rucksack:&str) -> i32 {
    let compartments = split_into_compartment(rucksack);
    let duplicate = find_duplicate(compartments);
    let priority = compute_item_priority(duplicate);
    return priority;
}

fn split_into_compartment(rucksack:&str) -> (&str, &str) {
    let mid = rucksack.len() / 2;
    rucksack.split_at(mid)
}

fn find_duplicate(compartments:(&str, &str)) -> char {
    let duplicates = find_all_duplicates(compartments);
    return duplicates.last().unwrap().clone();
}

fn find_all_duplicates(compartments:(&str, &str)) -> Vec<char> {
    let (left, right) = compartments;
    let duplicates = left.chars().filter(|c| right.contains(&c.to_string()));
    return duplicates.collect();
}

const PRIORITY_LOWER_A: i32 = 'a' as i32;
const PRIORITY_UPPER_A: i32 = 'A' as i32;

fn compute_item_priority(c:char) -> i32 {
    let v = c as i32;
    if 'a' <= c && c <= 'z' {
        return v - PRIORITY_LOWER_A + 1;   // +1 because a = 1
    }

    if 'A' <= c && c <= 'Z' {
        return v - PRIORITY_UPPER_A + 27; // +27 because A = 27
    }

    panic!("Invalid character");
}

fn second_challenge(data:&str) -> i32 {
    let groups = split_into_group(data); 
    let priorities = groups.iter().map(|group| compute_group_priority(group));
    return priorities.sum();
}

fn split_into_group(data:&str) -> Vec<(&str,&str,&str)> {
    let splitted_data: Vec<&str> = data.split("\n").collect();
    let chunks = splitted_data.chunks(3);
    let groups = chunks.map(|chunk| (chunk[0], chunk[1], chunk[2])).collect();
    return groups;
}

fn compute_group_priority(data:&(&str,&str,&str)) -> i32 {
    let badge = find_badge(data);
    let priority = compute_item_priority(badge);
    return priority;
}

fn find_badge(data:&(&str,&str,&str)) -> char {
    let (first, second, third) = data;
    let duplicates_first_second = find_all_duplicates((first, second));
    let s = String::from_iter(duplicates_first_second);
    let duplicate = find_duplicate((third, &s));
    return duplicate;
}

fn get_sample_data_str() -> &'static str {
    "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
}

fn get_data_str() -> &'static str {
    "QLFdFCdlLcVqdvFLnFLSSShZwptfHHhfZZZpSwfmHp
rTJRjjbJTgzDJjdsRsfwtfNwtfmZpZNhmmzt
bMdJjsjglnVMFCCc
BZvZMBBBMTtZTgcCPdgtgQCrrV
VspNDDpsGDGnRmRpRplQdrrPcPCjgDCcPQCQQj
RVnsmspnpwFRlHGRwHHlnRSThSSvBTbFTZMqTMZMTZFh
ttffrVJWtWpgtQnZGVnNSLTHSZ
jRsjjmhdRcjcRsFcdwGLMZSnHMTSMNZN
RjczlvjhPCcPjcvRpbglWplJBblqrGgq
NwCWwdNQNDCwGpWNQwmJtZgPZrdJgLZcPhLddr
blqpnFTqrLbcLPtV
MnjqSSpqlFRvSqNDGHvWHQDwfWmm
jfLljlQhDLmlrMJQtJTJrQqQ
NpBbjjsdMCgCCMrb
dwspwGnSHHGsGzDDlFDjVWjfZWnP
wQhTZwvpZFZdqWLnnwSrWC
mfDmMFlDcPLdgDSCLCqg
PmzclsMclMlFsHHsJZFHpT
LfLJWNdJnBLfhndfWdZqcgDZgSqgCCSSSLDF
bQVQmrrjPqQDZSZBCQ
RRtGjVmRsPbPrrnNNpNHHnBnpHns
PfbGNwGBwNcPTbGNQFBVjsjztVztVjHV
hrdCJhmlJhZrLDRmghrmDrzqFsbgtbHqnznzznQtbjtz
WdZdDJCDmrJmLZrLDLDZlClcSccwSPbNPfSNWfGNNWMGNc
QwrnTSgqgFShSdfHPdbS
BGdjmMmZMvfhvCZZPf
BzGmzVGGGzmzGpVBtdnQqqdTQQDDqrpR
PPRPwLQlLtbPmbwgJhrSssNlhhrgsZ
fFTdFvTMNfzVnFqdnjgSSjjsSjhghpJs
dvczcFzNTWVWMFLcLbQQwmbHLCLL
HhLLDfMmTjsjmLmhsmmfZMjGtpGJtdcvnCWtZJcWGddttW
gwrwwgzwgDpJddrJDr
SBwBBbgVsVmRRhDM
SZdmfdZRTQZTQgHVVGGRqZdVCjCcNCNcJRWcCBbJjCPCsNsc
FnhzMMhDDFlzlnvpwMLrMDCsbcjNJbcJbBcBfPhCNbWj
wzwnpDDnLvFnLlttLrzGgTVGQqZtTqSqSdfZTg
FJJWWWrCGWdmzFlTVqqlMhmvVlNh
btpgtfjZjjhgggrNMThl
DpwpfRZDZwwfwjsnjfsZnnnwGcCRCHcCCCLGHLWcrcFCWCHW
ljHHHBtjQthchhZpqqNt
FTmJnPFwzlJPmzTgTgbFwJbMCpbMchhhqbhWCDMDhZcppM
JwlFGwVGnFFGBGjdSsfdsG
QsvpGhjGgswvjjwjjjvPpThJfLZCCLCSSLbFLStCFCSgtH
mDrzdzMqqnMrDMmZnqnzNVRCStlCHFSCtJqlCLFCFLfJfJ
mBDzNRWBDDMBpGsZcGZWGjPp
SlLQhQsvvttFlWsWcfHHMTJfwSHwTfTf
VZmmrRDRfpTHJcRf
jzBnDDgjPchWlsQsBW
LTLVdTSLNTLnTSnrWvdwswsfmJwmwGsffH
FbgCbRRppCpPbgMcZvCcGftGGltwHwGtplQQsfJw
CRBMCvZZRgMgBbDCPcDrjLzWLVrSSzShSSNrBS
hVJJjhjRVRZjQvDfBstsBVNBdwstHsld
pCTCcMqCThTFLFFPWcWSPHtwwdmcBHHmNtHdmwmwBl
rMTCCWPLLPCMFhDnDrjzRrfDJD
pqMpCvMchvFNWSBdVNqQ
zDRJJDGJJtNtmGRRWVdFWWVdSfjb
DDJLmnJmzwGmGLTPhTCNpgcrpv
cpPpbPWVprWcbJrrwpCwwdWrvNNFRqzNnChgqFzFnZvqFlzq
fTtHLfSHSsNDGLSmsLvnhFqzhzlzDhhvRlFz
mSMLHTQTmHMSfBSMTPdBJJNNVddrVrbjbJ
zpCpBTnFgFbncznbblzdhRswdlJsLllJdw
QqqmtWVPWvHDVmqDhjsljwRhlZldhRMQ
tWSHDmVfmrtPHVgGRRbgTRpSgpTc
ssTbzFRtPRwTFZtvbPRMhndBqMMvMBHJnnHMMd
WQVWzlGWVBqqdMQJMq
pVSpSSgLfjDzWrLGWWjDzzfLtbRFFNtPZRssspsNRZcRsNZb
jnPzzGlnnznWnzhvGnnpVFrZmVFcgjrrmZRFtj
fsbgTdwdqBbfwCptVtdZRcrRCp
gsMbgfHsBSwsGhhJWMLnWPPJ
bpmbJpNbbbNGGmRJzJTsfdsvsNdglfhssCvC
hWLwQjZjLhjHFFBLZldvvflrvtfjCrfjlT
QWVQZZFDcDJJhJJc
RmRghgRlNgfGGRmdGqhsgsZFZZpBvHpZppHcgH
tbLCDLnLtSbbbjtPtMLtDPTvHHBpcHcsHvTcHsmZcF
rSLrMJzDznzGmhNlVwdrRr
vWjljMWcnSSpjmzbJVzJrTCmtGJV
NZDDQLRqPJrPzrprTC
gqDqqwpdHWhlgnjH
ccptcpstDvbNvHbLNRZZ
dFjhdnjQFJlFCQSjgngJPJgWWrRNWNRtNCzrVbRzNVzZZL
PhThFSPPSpsmTcqMwt
cLcLlMhGMGcpGzslHFHFvnHlBDvWbT
VHdQwqPJdPwjJPdPQRrmjjjQnTFrbvNWFFffDvbvvNDvbBNN
RwRJCHmmdQJZZLzGphcCtz
hVvFVjvjVWmFRQVZqTpqtwQpwpqZfp
gvDlSBDJSlPLcLdDwzwtptqTTTzwcCCt
JgrJGbLgvnWsmvVr
rwmqqRqrnHQGmnjCCqCzdBzCBJBz
hFLgbWWPWmvtLhPtgpcdjJdcBJdpJjDsgp
lvfhSSPWtTNTTZZmfr
bHDDssRHsjNMbJjJLQJsbTtGvSCzCGQCTzGvSqSBzT
mmVrwhmmpfPStnTSBnhStG
pcwrptZcgFcmpgHRDjjZlDJsjbbD
JJRrmFqJMdFFJMjjJcqGgzSCSHSCscPCHPHGZc
VWpWptnvSmpPGCHC
vQnDLBmbntvLBbnlldTQFlJlFFrNRd
LPDftnHFQfwmBcBGmc
CVqRsdqvdrlsCVsNvqwwSpTNSSDSDDBBTTSN
lqlDRddjbblRbRqrlRRjsbvghHHnPQWjHWQWZHPHWZhFnP
bwQsDcgsJqcsDpcQRQnpqtVSVvgSMMMfMvfVBVfdvM
CGZFrHHPrTZNGGZZHmCZHlVfjfzjSfzBtBBNSBVjvntf
ChrCCLGrTlhJnhDncRbp
nmFnhfTQjSzfjddZWsRRRFRFGl
HDgCwgtQbZlHsrqHHr
cJPCgCCPbpbgDMPvMQnjmnhTfmzLpQQmjz
QFHSQdNMCSgcSgFtttPNFJpCpnTjZlbblpppZplZjz
LqLsWMRRfrrWMmMGpbTnbppbZnTjpnmm
fWBrMqWsGswGfGRMMwrLgtPPdNFBQPHNHNPPcFPP
dngbSppJSSppbVMZQQMjqfQQgwcl
TWmSWtvCRCWjwfjqQqMstq
FhFvRzSTNmhHnVPhGhBJdBpB
gcHPgzGmPPwTsSTsbwbdWD
QjBLLfVhhBqqBFQLrLjVFlNpNDtsSWTDdNptdbqbdS
jCMFLVjFBFjJJLFFMVBFrLnvPzHRmHPGnGWWcCvzHRZm
PDPqWWjhPpPbCsjwjTVbLT
SrtCttGRMddSVwHFSs
JtfvttmrGMRRJzJCqhqqqWQZhCNgJZ
ChrCVFQCVQlwQNwpQcmmcjmWBmddghjjdW
sbDTZStTqqfSBggPmWjWWNsL
THqqSHDTtZTDTHZZbHTzRzFvlFCVprFprQVnCzNppQrz
PdfWCwMWjPSrdgCMnnlGsGQvvpJZvFGnps
DmBhVBLbbVqVBzTRLBRzzTLNNpRQNNZQZppZvlQpZvllvF
zVDtVHBbbTbzDbrjgWjMPtMWPMlj
JLsTTNDsgTMNvDQpLpGpLGNJShrfzCFnSnSrnfzCfTFhWrfw
ZcqrRddHZZVRfzWnVWCzWFnn
tZHZtrHHPdRtdHlcccQggsplpJDNvMGNGMss
cMCLfStfMTCjPMPcGzjftMbgsRNmRgmmGsmnJbNJbghJ
QHVVWrFFWZNShHSgbSJm
qZwwrrpqpZpZFvqrQdFlQVSwLBMBfTTLTjLBCcdTMzMftPPB
SwsdBTvgvJLPNptpCpCmBDtn
wffrzwGFWFNZWpjWZnNm
zrfflbRwJPhbPbsS
HjHHRtwjnjRblQRttHwQGvGWNNBWvqGzfTvfNN
FmScCcrsdVZrpBrVcCVFzffvzzmWGLWgqWqgGWzW
SFVSDDBdsdDSJhnjJltJbPtHRM
FjGFVqWrzQFlQrZzGQzFLTvfwwTgMnvcnbRMLRdnfb
CCttSNsSnRfgncSg
CNspmDBPtPmJJNBJPNpDhQZVzQlhqrGZflfVjQFrQj
dNNdHWcmdmPPptmmWHpPTFFjJPGrQsVsPQGGGJVDrVVGrS
MhZlZhlgflgfnfDtjbjJGbtnVtGS
LtZqlzhzqMZWHHLwdHmFWp
llNRlfwWRwwLlwFNNgRrVCBjdjCVdjpWjtVWCD
HTQqzPqzQPmhhmSPznSsssJtdnMZddtMCjprtMjCnBVnjZ
PzHQmqsGSJPSmQqPbfwNcgNbNgNfBGwR
lPdzlZPzQzMZQGQrTZvvpjHTTpfsTTZb
zRShhtWRnqnqSNRnDTTHvfNJspNsLpTsjL
hBVncVtDSnhDnDBBtGrlzwmmMlGmVrMdrP
HPTZVHVPlHDPlfgnjJFdJdjPjSPqCS
hLRRBhwGhqbtmsRSSSjjdMJjnJGSMj
QrQtqrRrcQDgVglc
ZTwbbZdchZZjmVWHTrHWBVJtBB
glslCDqLLDfGRqlsgLssfrCHBHFHmrHBBppFmCJWWp
fRzvvvgGgNSNvmQbSQ
qPGGPwCTqTzHCvPGqWdLFLssLpstLLspvd
njJchhcbjbDrbcLNlLrpWWrLLHgp
DQhMMMJQMQJnVbbnRHSMPwZmGZPZRTRCwTZmZGwz
zzGNfPbcgdPqLrqvWWVzMq
DGmJtnJTJRhhJMhCQqCLCLrrLM
ZnHDtSZlTBHnBdccGSfGcwjjdb
FpZDpQZDvMwZpCCMdCBPpJGPPLgJGGLffJJL
jlbswNrlPPJJfGlf
bnNwqbHnNwRSrqhbdCcmHddQzddFDdvZ
gbQQQngWPVVtvvPQNVNvWWSHGwDsCCmDtHSlmrssDmHs
fqhMLFFMMZqZMRZqMjRMqLJSCdFlrrldsDrCsDSSHHGCSC
MJRZLZLGMcTqczjNPzNnzPvWBVgNnP
gqdbBffTvlRHbwLl
nMMQJQpGdsFpQsJzNMRLLDlmLLmjFFmLjDRF
pzGMnVcMBfTdtBBV
WSbfmrrrrWdbWmdfDSSStmHjtMtvCLVnqBHCVGtVGnMM
lRcgFRZhJgnMLjvGgv
lcvwTcFTplvwphzcTTJTbsdsPSPDdbmzmDSWPsSm
bbdTjTQTQMsZNqqhJrZslg
jFGVjwfCPVGfwjCVqWhWZFgqWrglllNN
PjfSPzRBjCCfSBCGBLznTndHcdMLbMmmdT
wSVMJSVccdGwGnsgbVTTbRsCRNgN
rHjhHLmrhPJrqjNTRDgBbbRRRs
zqmPPqqpPLzltrMdJcZpfdpGWWJJ
ZhrBBJGrgJhGHttGGVPPcPPF
cnzLqNssfRnpfWqsLfcfWQNMbMVPDtnDtbHFtMbPtVPFFM
jfqzCCLsWQLcjgldjmljmgTd
wghGSSGZPVwgqtwtwCCtFFMM
BvbspnBznvvWHWHHHbCQptQFQlFcqMClqLLq
JWzzsJHWzfWjJrvMBWHBBGDmVDrVhZmmgSPSmZVVrh
ccRMJRsjjgJgcPCSCCVCwsSWVNzp
WQQqnmrBWtqWqdSbVwwBSpbbCSBB
QvDqmqqmgWPWjPvW
msqpjDWspRWwvFvDWWhnbbJfPzFQblJJPlnz
gGGrMTgLVBsBBLdsVTrSCBffHQfdHhnbPPPPffndlbzh
ZCVsCGSScsLZpwNpqmZRqW
PPsGmJPVPQPZmsQCVPJPnPCMDcTcdqDDTqvFhvnTjRDTDchq
BdrtzNBLHStHrdrlwfNThvFhcvbDccThjbFBqq
SSgdHNfSHHgzLHtLNWSPQQPMQVpmmppVCmQZCW
pPssrWWLdndHPJdd
QNQFTLNBFTzzgjfGTjffFNZjCSGnHDnSDJHnDScttDCcDnmd
FVzVLZwZZgswqqrbphbR
VpWCZjCwWnppZpqnhNjjNZjFLtLzQJHdHLQRzWLRzRztHJ
DMGPmPMgTSmsgQzRFbdHRLJgdn
csDMPMGDDvMSSPnDTvrDChhwljlqNNjchNCjNVcf
WpGGmbSGpVWWpjMMTNdfCFNdFfRNwNSF
JsQztzrvrJqsTTRbbvFBhhhv
cLrDqLccsLqbDHGpZWDHgjGlZW
QGMQJMmsJmMCmmqjsRvLvvdgvgVvDVdD
BDcrcNbNppwTpzRdvvchhFvfFv
plBBwWrbpQHDjGmGJl
mzFlTdmSDzrPvCJqqDVVNC
hfRmhgjRhnfwnRHcnhGGvPJQPvvfLfQvNLGv
BhhnjMgRWghpwjRWMRjrZzdbSbsdstTrltdmMs
bLLnbqjpvplnDvNlqpqBWJZSdPJCNdJJThhSPhTd
HFwHHQMMFHGzGwRPPJPTWthTZtJSQr
mfWMHFHWHmgmFcwGwwpbDljqjBDcDnLcVnlb
wBrWBwSWRJMBwdZnPQPgFnwGVF
fLjfbsvDDfvvqqGqZGqmPQgqTGGG
vZLsjzjjZCzJWRNSBR
jTRbRHHqPqTRBHqdjhgvgghhZQdDvvgvhC
WLWWzzFszsmNFGWSFmMrpghCtZvhlQNDgQCDgctC
FJsLsSrDmsFSDLWrzJmmMsGqjRBVbJTBVPVBbBqRjPBjHn
QbwwnDDQDcDfSbDbfhhrvrCtJMvJSCvvJh
FWRjjLjmdZWdWNBFNWNlNQQrMGvvMGgssGvQRvrMJs
BjWdlBpmdmBWFWdpWfPfpVnVwfHpqPQDbq
SqrvlMldqvSWdGPTGzWpWpzpHP
tRwmhtbsRRFsLwGGTVDHppTNdbVp
FRCRQdCFtCLmBhCcmmQdhFdCvnfjffjZlZnjSnvfcSrrgMgn
GQQtNJQWWcqPPhMMtwqD
WpWLlBWZCvhjwMMZqDDP
WgvmLVmHCbpppLgdllHddvCmFGzGnfsJJQJsJncSsccFVffF
HcSsSlTTvvPPWWNMWWgPTPPbGbbrwJQbrrDphrHJJRpRhp
ztfLqqzmRwDGlLDb
fdVtmqjdZBmSvjsPSWlTgv
DPvDhhMRRMhRNDLPMNsbwHwrjgnddqddrWdPtHzr
pcBGSpcVBfJWCcmJGGwHtzgrrtwqzdrtrngG
mllBlBZmMlQWRbQv
SGZBSFMZllJWmzvfpp
NTqbNrhHNHWgNqHrNhNQbbjHJLcnfnzLLnLmfcfccJcfQLcL
HggbTNRRTHqqbVSGMSZVWDMDwVPs
SBsSlvbPlFPvRlbPsMFZLgVLrLsJVgzrCJfVCH
jcNddNdGzZrVgNVJ
tTGwdcmWGdtwQmwmwZdwSlhBPbhPTBFRhlhSMFMR
RzStzTzzvvQvSHVvhVgBqMMFqhPM
ddlLLwNVLWLjbbLrjrbWrwmlhcFmBGgFMMPgBcGBBqPhggMs
dLwdVCVWWdfNwNwLrWrbfbJNptzDDHRnHptHtznHTppnQCtR
RzcfMBHLzpDQFmnDSWNB
dbqjtjVqJZZGjPGJCPGbPndNNDglrmQmNSDgSlSSng
hjCTqhCJbhVCGNvMcfvhfRLhvchz
sDDqDMtqshJhPvhhCpSCCWlZHSWp
bffRcbBGGTwGfGfbNjgSHZSgWwplHCClZZ
RTQBbcnbRNmGbGTQLbmbJVqLllsDVMsPDVVvttMd
nbLBjnqwgfRRBgBwnllbLlwScvPdZPcScZPcdFZJPvZPvcMZ
tChQpphHrrHztssZdcDJcPZcMvWv
hpTHVMQMtQtVpzBfwjfRnfwfnjVl
ljJlvvJQlrlcJcWpPzgthnPnzMgpgSpC
smtmZBmHZTVttHmqFqmzCSZSdndzShPNgPShgP
bVqFHLqLqfHHFwbBLHcwDQrDrtjlQvGjlRQQ
pwhVsPvVVCFtmhPhzqGqqZMZvGTTTMlGWM
drrrrDfDRrNQdQdrRrBdjGWqWqWlGlGtlGbGZGBTLc
tSDfgnHrdDtVSPSshJCSPh
WlWlDqhglLhsdgrcbFdJJpPpdBbB
ZQZvSvzRMSzjZjvZmMMpbFPQFVBrVbPcpbJFLB
SwGZmjvCRSMRjMzZvRnstHftNfswHsflLhNWHf
jsprCvGRQrtjCsQrGsrzvGHhgmHVmHZgggmMGVmhMbHm
FFFdDSdwSffJWqqMzzMmDVbZ
LLcdcfcfPwwBzdTTdtvlsrjCtvPvprnsjR
MvtSqNSWMzjwzFTD
ZRPlcRpQszNgszNwVT
bcZcrcPlcPLLLZllPlbcbLSBfWCvHvWWNSmSqNqfWN
rNdZpMGnddgggwHwzRPCzDDD
vcvhcTLhZLhLPCPHPDPPVvzH
LTmBmthWBchWLttttFJFLlFnGJNsfpdjNsnMnMpnpZdssn
ZHWFCvqBDdqqqCTDHHBWrgppTMhhVpspMPQcSgQVPS
jblbGffntRwltfMQVrrQscphfg
ztJrGtbwGztbmtzzRGnRznWNNCWmHHdFHdFNWWHHqCqZ
WGWSSZvVvqmrmzPm
NgjtwFFlwDsFghNsMtlcjljcPqrQHcZzQznpQQprnqqzHQ
tgMCwNhtgbdLZRbZCT
PQSPQrSGZnGnVFhpVhRRlvLvBDRV
tjctcjTMMpDTvFTlRD
JCftsccFCcmsJJGZGGmPHnQrGwGS
TrjRFFRnpnRCHNFSjSRrffJvJfzqQBsjqQqzzffd
ZtlgMDhZhgmGDLVZLlGtLPqdQQvvfBJJqzzBPdMzdd
VlLDgLLDWtGZwgtRNTNrFTqCwqHTrr
LpcDFDMMPjMLLjpcDGCHgHssGHWnbCBWBHvm
QfZhrhVVdZThlZlfVvVzZrTbgQnBHsCCHgJBsCsJBHmBmn
wwtvfZztlTVlhtrzzlLNpFFRjMPDpRcPFwRj
VzZhhQHQJJWJSSFWDGclbmNPgglPgVGc
ddBTqCjjBCcrqrCRrwGPGmmDGmbpBGNpNNgg
CRMjwsjwsLdLRrQFJSvMFMWZcHFW
JgJJPvtrhRPQQzSRMQFFSF
BLqsjsdLsMBqblnsGbBqVqdwSQSCSWwNFwczQWCNNwNCHn
ljqbpLbbdDlbDbqDDVtMttTTgpJJgThhJrJr
nflndmjbSnlTQGwvWGPHGRGj
NtstcMcDJMvwgHfFvDgR
qqqpLrMsLLqLNNnzbrdlbZSrznfz
ttZCCFjNjnPVCFQPPFbbStrzqzqrrrcwtmJJ
gTTMRMTWsTGGTddHTTbBzBLSmqbbJGzGmqqb
HpgpMTvRhHHTRDhMsHdHDRhjJlVPJjNFJnnFpQQVfPCjnP
VqJVQPpjQqPBbHwldmLfVVmd
tMGvrzzDGCDDddwLbgLvLwcm
TWDWCzTZDGMZtzWWtsFhbRRqRQRjhbNQBBTh
zgLgLHnnzCCvnsHSsZBZBsTRdD
rslllhJjcQNNGjpWJlSRTRdwBVSSNTPVSdPB
jGrGqjJfqccrfqGcGplrJpFvzggqmCtMzmsMnvMvvCgm"
}