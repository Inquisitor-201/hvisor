// Copyright (c) 2025 Syswonder
// hvisor is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR
// FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.
//
// Syswonder Website:
//      https://www.syswonder.org
//
// Authors:
//      Yulong Han <wheatfox17@icloud.com>
//
impl_define_csr!(MerrSave, "Machine Error Exception Data Save Register\n\
                            This register is used to store data temporarily for the system software.\n\
                            Each dava save register can hold the data of one general-purpose register.\n\
                            Two major causes contribute to the motivation of an extra SAVE register for machine error handler.\n\
                            One is the unpredictability of the machine error exception for software,\n\
                            and its potential of interrupting in the processing of any other exception handling.\n\
                            You can simply consider this as yet another scratch register.");

impl_read_csr!(0x95, MerrSave);

impl MerrSave {
    pub fn data(&self) -> usize {
        self.bits
    }
}

pub fn set_data(value: usize) {
    write_csr_loong!(0x95, value);
}
