use mmu::MMU;
use timer::Timer;

pub struct SystemData
{
    pub mmu: MMU,
    pub timer: Timer,
    pub input: PlayerInput,
    pub width: u16,
    pub tile_width: u16,
    pub height: u16,
    pub tile_height: u16,
    pub clock_speed: u32,
    pub horizontal_sync: u32,
    pub vertical_sync: f64,
    pub cycles: u8,
    pub debug_flag1: bool,
}

impl SystemData{
    pub fn timer_tick(&mut self)
    {
        //println!("@@@@@");
        self.timer.cycle_calculation(self.cycles, self.mmu.div_reset);
        self.timer.update_registers(&self.mmu.mem_map);
        if self.timer.mod_delay && self.timer.timer_counter == 0
        {
            self.timer.timer_counter = self.timer.timer_modulo;
        }
        self.timer.mod_delay = false;
        //self.timer.divider_tick(self.cycles, self.mmu.div_reset);
        if self.mmu.div_reset
        {
            self.mmu.div_reset = false;
        }
        self.timer.divider_register = ((self.timer.cycle_register & 0xFF00) >> 8) as u8;
        let overflow_flag = self.timer.tima_tick(self.cycles);
        self.mmu.mem_map[0xFF05] = self.timer.timer_counter;
        self.mmu.mem_map[0xFF04] = self.timer.divider_register;
        if overflow_flag
        {
            let mut value = self.mmu.get_from_memory(0xFF0F, false);
            value |= 0x04;
            self.mmu.set_to_memory(0xFF0F, value, false);
        }
    }   
}

pub struct Registers 
{
    pub accumulator:u8,
    pub flags:u8,  
    pub b_register:u8,  
    pub c_register:u8,  
    pub d_register:u8,
    pub e_register:u8,
    pub h_register:u8,
    pub l_register:u8,
    pub stack_pointer: u16,
    pub program_counter: u16,
    pub interrupt_master_enable_flag: bool,
    pub interrupt_master_enable_delay_flag: bool,
    pub halt_flag: bool,
}

impl Registers{
    pub fn new() -> Registers
    {
        return Registers 
        {
            accumulator: 0,
            flags: 0,  
            b_register:0,  
            c_register:0,  
            d_register:0,
            e_register:0,
            h_register:0,
            l_register:0,
            stack_pointer: 0,
            program_counter: 0, 
            interrupt_master_enable_flag: false,
            interrupt_master_enable_delay_flag: false,
            halt_flag: false,
        };
    }

    pub fn mapped_register_setter(&mut self, index: u8, value: u8)
    {
        match index
        {
            0 => self.accumulator = value,
            1 => self.b_register = value,
            2 => self.c_register = value,
            3 => self.d_register = value,
            4 => self.e_register = value,
            5 => self.h_register = value,
            6 => self.l_register = value,
            _ => (),
        }
    }

    pub fn mapped_register_getter(&self, index: u8) -> u8
    {
        match index
        {
            0 => self.accumulator,
            1 => self.b_register,
            2 => self.c_register,
            3 => self.d_register,
            4 => self.e_register,
            5 => self.h_register,
            6 => self.l_register,
            _ => 0xFF,
        }
    }

    pub fn mapped_register_setter_with_flags(&mut self, index: u8, value: u8)
    {
        match index
        {
            0 => self.accumulator = value,
            1 => self.flags = value,
            2 => self.b_register = value,
            3 => self.c_register = value,
            4 => self.d_register = value,
            5 => self.e_register = value,
            6 => self.h_register = value,
            7 => self.l_register = value,
            _ => (),
        }
    }

    pub fn mapped_register_getter_with_flags(&self, index: u8) -> u8
    {
        match index
        {
            0 => self.accumulator,
            1 => self.flags,
            2 => self.b_register,
            3 => self.c_register,
            4 => self.d_register,
            5 => self.e_register,
            6 => self.h_register,
            7 => self.l_register,
            _ => 0xFF,
        }
    }

    pub fn mapped_16_bit_register_getter(&self, index: u8) -> u16
    {
        match index
        {
            0 => ((self.accumulator as u16) << 8) | self.flags as u16,
            1 => ((self.b_register as u16) << 8) | self.c_register as u16,
            2 => ((self.d_register as u16) << 8) | self.e_register as u16,
            3 => ((self.h_register as u16) << 8) | self.l_register as u16,
            4 => self.stack_pointer,
            _ => 0xFFFF,
        }
    }

    pub fn mapped_16_bit_register_setter(&mut self, index: u8, value: u16)
    {
        match index
        {
            0 => 
            {
                self.accumulator = ((value & 0xFF00) >> 8) as u8;
                self.flags = (value & 0x00FF) as u8;
            },
            1 => 
            {
                self.b_register = ((value & 0xFF00) >> 8) as u8;
                self.c_register = (value & 0x00FF) as u8;
            },
            2 => 
            {
                self.d_register = ((value & 0xFF00) >> 8) as u8;
                self.e_register = (value & 0x00FF) as u8;
            },
            3 => {
                self.h_register = ((value & 0xFF00) >> 8) as u8;
                self.l_register = (value & 0x00FF) as u8;
            },
            4 => self.stack_pointer = value,
            _ => (),
        }
    }
}

pub struct PlayerInput
{
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub select: bool,
    pub start: bool,
    pub a_button: bool,
    pub b_button: bool, 
}

impl PlayerInput
{
    pub fn new()-> PlayerInput
    {
        return PlayerInput
        {
            left: false,
            right: false,
            up: false,
            down: false,
            select: false,
            start: false,
            a_button: false,
            b_button: false, 
        }
    }

    pub fn update_input(&self, system_data: &SystemData) -> u8
    {
        let mut input_value = (system_data.mmu.mem_map[0xFF00] | 0b11000000);
        let option = (input_value & 0x30) >> 4;
        
        if option == 1
        {
            //println!("{}", option);
            match self.a_button
            {
                true => input_value &= 0xFE,
                false => input_value |= 0x01,
            }
            match self.b_button
            {
                true => input_value &= 0xFD,
                false => input_value |= 0x02,
            }
            match self.select
            {
                true => input_value &= 0xFB,
                false => input_value |= 0x04,
            }
            match self.start
            {
                true => input_value &= 0xF7,
                false => input_value |= 0x08,
            }
        }
        else if option == 2
        {
            //println!("{}", option);
            match self.right
            {
                true => input_value &= 0xFE,
                false => input_value |= 0x01,
            }
            match self.left
            {
                true => input_value &= 0xFD,
                false => input_value |= 0x02,
            }
            match self.up
            {
                true => input_value &= 0xFB,
                false => input_value |= 0x04,
            }
            match self.down
            {
                true => input_value &= 0xF7,
                false => input_value |= 0x08,
            }
        }
        else if option == 0
        {
            //println!("{}", option);
            match (self.right | self.a_button)
            {
                true => input_value &= 0xFE,
                false => input_value |= 0x01,
            }
            match (self.left | self.b_button)
            {
                true => input_value &= 0xFD,
                false => input_value |= 0x02,
            }
            match (self.up | self.select)
            {
                true => input_value &= 0xFB,
                false => input_value |= 0x04,
            }
            match (self.down | self.start)
            {
                true => input_value &= 0xF7,
                false => input_value |= 0x08,
            }
        }

        else if option == 3
        {
            return 0xFF;
        }

        // if (input_value & 0x0F) != 0x0F{
        //     println!("system{:02X}", input_value);
        // }

        return input_value;
    } 
}

pub fn get_system_data(emulator_type: &str) -> SystemData
{
    match emulator_type.as_ref()
    {
        "CLASSIC" => return SystemData
        {
            mmu: MMU::new(),
            timer: Timer::new(),
            input: PlayerInput::new(),
            width: 160,
            tile_width: 20,
            height: 144,
            tile_height: 18,
            clock_speed: 4194304,
            horizontal_sync: 9198000,
            vertical_sync: 59.73,
            cycles: 0,
            debug_flag1: false,
        },
        _ => {println!("NOT VALID EMULATOR TYPE");
        return SystemData
        {
            mmu: MMU::new(),
            timer: Timer::new(),
            input: PlayerInput::new(),
            width: 0,
            tile_width: 0,
            height: 0,
            tile_height: 0,
            clock_speed: 0,
            horizontal_sync: 0,
            vertical_sync: 0.0,
            cycles: 0,
            debug_flag1: false,
        }},

    }

}



#[cfg(test)]
mod main_tests
{

    use get_system_data;
    use SystemData;

    #[test]
    fn passing_bad_data_to_get_system_data_returns_empty_struct_data()
    {
        let system_data : SystemData = get_system_data("");
        assert_eq!(system_data.mmu.mem_map, vec![0; 0x10000]);
        assert_eq!(system_data.width, 0);
        assert_eq!(system_data.tile_width, 0);
        assert_eq!(system_data.height, 0);
        assert_eq!(system_data.tile_height, 0);
        assert_eq!(system_data.clock_speed, 0);
        assert_eq!(system_data.horizontal_sync, 0);
        assert_eq!(system_data.vertical_sync, 0.0);
        assert_eq!(system_data.cycles, 0);
        
    }
}