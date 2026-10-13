import struct

class PE:
    def __init__(self, path):
        with open(path, 'rb') as f:
            self.data = f.read()
        pe_off = struct.unpack('<I', self.data[0x3c:0x40])[0]
        num_sections = struct.unpack('<H', self.data[pe_off+6:pe_off+8])[0]
        opt_size = struct.unpack('<H', self.data[pe_off+20:pe_off+22])[0]
        sec_table = pe_off + 24 + opt_size
        self.sections = []
        for i in range(num_sections):
            sec = self.data[sec_table + i*40 : sec_table + (i+1)*40]
            name = sec[:8].decode('latin1').strip('\x00')
            vsize, vaddr, rsize, raddr = struct.unpack('<IIII', sec[8:24])
            self.sections.append((vaddr, vsize, raddr, rsize, name))
            
    def rva_to_offset(self, rva):
        for vaddr, vsize, raddr, rsize, name in self.sections:
            if vaddr <= rva < vaddr + max(vsize, rsize):
                return raddr + (rva - vaddr)
        raise ValueError(f"RVA 0x{rva:X} not found in any section")

    def read_qword(self, rva):
        off = self.rva_to_offset(rva)
        return struct.unpack('<Q', self.data[off:off+8])[0]

    def read_dword(self, rva):
        off = self.rva_to_offset(rva)
        return struct.unpack('<I', self.data[off:off+4])[0]

    def read_string(self, rva):
        off = self.rva_to_offset(rva)
        end = self.data.find(b'\0', off)
        return self.data[off:end].decode('utf-8', errors='replace')

# pe = PE('F:/SteamLibrary/steamapps/common/hackGU/hackGU_vol1.dll')

