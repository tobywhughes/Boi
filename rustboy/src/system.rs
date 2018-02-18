pub struct SystemData
{
    pub mem_map: Vec<u8>,
    pub width: u16,
    pub tile_width: u16,
    pub height: u16,
    pub tile_height: u16,
    pub clock_speed: u32,
    pub horizontal_sync: u32,
    pub vertical_sync: f64,
    pub cycles: u8,
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
    pub lcdc_register: LCDC_Register,
}

impl Registers{
    pub fn mapped_register_setter(&mut self, index: u8, value: u8)
    {
        if index == 0 {self.accumulator = value;}
        else if index == 1 {self.b_register = value;}
        else if index == 2 {self.c_register = value;}
        else if index == 3 {self.d_register = value;}
        else if index == 4 {self.e_register = value;}
        else if index == 5 {self.h_register = value;}
        else if index == 6 {self.l_register = value;}
    }

    pub fn mapped_register_getter(&self, index: u8) -> u8
    {
        if index == 0 {return self.accumulator;}
        else if index == 1 {return self.b_register;}
        else if index == 2 {return self.c_register;}
        else if index == 3 {return self.d_register;}
        else if index == 4 {return self.e_register;}
        else if index == 5 {return self.h_register;}
        else if index == 6 {return self.l_register;}
        else {return 0xFF}
    }

    pub fn mapped_register_setter_with_flags(&mut self, index: u8, value: u8)
    {
        if index == 0 {self.accumulator = value;}
        else if index == 1 {self.flags = value;}
        else if index == 2 {self.b_register = value;}
        else if index == 3 {self.c_register = value;}
        else if index == 4 {self.d_register = value;}
        else if index == 5 {self.e_register = value;}
        else if index == 6 {self.h_register = value;}
        else if index == 7 {self.l_register = value;}
    }

    pub fn mapped_register_getter_with_flags(&self, index: u8) -> u8
    {
        if index == 0 {return self.accumulator;}
        else if index == 1 {return self.flags;}
        else if index == 2 {return self.b_register;}
        else if index == 3 {return self.c_register;}
        else if index == 4 {return self.d_register;}
        else if index == 5 {return self.e_register;}
        else if index == 6 {return self.h_register;}
        else if index == 7 {return self.l_register;}
        else {return 0xFF}
    }

    pub fn mapped_16_bit_register_getter(&self, index: u8) -> u16
    {
             if index == 0 {return ((self.accumulator as u16) << 8) | self.flags as u16 ;}
        else if index == 1 {return ((self.b_register as u16) << 8) | self.c_register as u16 ;}
        else if index == 2 {return ((self.d_register as u16) << 8) | self.e_register as u16 ;}
        else if index == 3 {return ((self.h_register as u16) << 8) | self.l_register as u16 ;}
        else if index == 4 {return self.stack_pointer ;}
        else {return 0xFFFF}
    }

    pub fn mapped_16_bit_register_setter(&mut self, index: u8, value: u16)
    {
        if index == 0 
        {
            self.accumulator = ((value & 0xFF00) >> 8) as u8;
            self.flags = (value & 0x00FF) as u8;
        }
        else if index == 1 
        {
            self.b_register = ((value & 0xFF00) >> 8) as u8;
            self.c_register = (value & 0x00FF) as u8;
        }
        else if index == 2 
        {
            self.d_register = ((value & 0xFF00) >> 8) as u8;
            self.e_register = (value & 0x00FF) as u8;
        }
        else if index == 3 
        {
            self.h_register = ((value & 0xFF00) >> 8) as u8;
            self.l_register = (value & 0x00FF) as u8;
        }
        else if index == 4 {self.stack_pointer = value ;}
    }
}

pub struct LCDC_Register
{
    pub value: u8,
    pub display_enable: u8,
    pub window_display_select: u8,
    pub window_enable: u8,
    pub tile_data: u8,
    pub background_display_select: u8,
    pub sprite_size: u8,
    pub sprite_enable: u8,
    pub background_enable: u8,
}

impl LCDC_Register
{
    pub fn new() -> LCDC_Register
    {
        return LCDC_Register
        {
            value: 0,
            display_enable: 0,
            window_display_select: 0,
            window_enable: 0,
            tile_data: 0,
            background_display_select: 0,
            sprite_size: 0,
            sprite_enable: 0,
            background_enable: 0,
        }
    }

    pub fn update_lcdc_register(&mut self, system_data: &SystemData)
    {
        self.value = system_data.mem_map[0xFF40];
        self.map_bit_states();
    }

    fn map_bit_states(&mut self)
    {
        let mut states = vec![&mut self.display_enable, &mut self.window_display_select, 
                              &mut self.window_enable,  &mut self.tile_data,
                              &mut self.background_display_select, &mut self.sprite_size,
                              &mut self.sprite_enable, &mut self.background_enable];
        states.reverse();
        for i in 0..8
        {
            *states[i] = (self.value >> i) & 0b00000001;
        }
    }
}

pub struct GPU_Registers
{
    pub ly_register: u8,
    pub ly_cycle_count: u32,
    pub ly_sub_cycle_count: u16,
}

impl GPU_Registers
{
    pub fn new() -> GPU_Registers
    {
        return GPU_Registers
        {
            ly_register: 0,
            ly_cycle_count: 0,
            ly_sub_cycle_count: 0,
        }
    }
}

pub fn get_system_data(emulator_type: &str) -> SystemData
{
    match emulator_type.as_ref()
    {
        "CLASSIC" => return SystemData
        {
            mem_map: vec![0; 0x10000],
            width: 160,
            tile_width: 20,
            height: 144,
            tile_height: 18,
            clock_speed: 4194304,
            horizontal_sync: 9198000,
            vertical_sync: 59.73,
            cycles: 0
        },
        _ => {println!("NOT VALID EMULATOR TYPE");
        return SystemData
        {
            mem_map: vec![0; 0],
            width: 0,
            tile_width: 0,
            height: 0,
            tile_height: 0,
            clock_speed: 0,
            horizontal_sync: 0,
            vertical_sync: 0.0,
            cycles: 0
        }},

    }

}

pub fn init_registers() -> Registers
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
        lcdc_register: LCDC_Register::new(),
    };
}

#[cfg(test)]
mod main_tests
{

    use get_system_data;
    use SystemData;
    use LCDC_Register;

    #[test]
    fn passing_bad_data_to_get_system_data_returns_empty_struct_data()
    {
        let system_data : SystemData = get_system_data("");
        assert_eq!(system_data.mem_map, vec![0; 0]);
        assert_eq!(system_data.width, 0);
        assert_eq!(system_data.tile_width, 0);
        assert_eq!(system_data.height, 0);
        assert_eq!(system_data.tile_height, 0);
        assert_eq!(system_data.clock_speed, 0);
        assert_eq!(system_data.horizontal_sync, 0);
        assert_eq!(system_data.vertical_sync, 0.0);
        assert_eq!(system_data.cycles, 0);
        
    }

    #[test]
    fn lcdc_register_test() {
        let mut system_data : SystemData = get_system_data("CLASSIC");
        let mut lcdc_register : LCDC_Register = LCDC_Register::new();
        let values = vec![0x00, 0xFF];
        for i in 0..values.len()
        {
            system_data.mem_map[0xFF40] = values[i];
            lcdc_register.update_lcdc_register(&system_data);
            assert_eq!(lcdc_register.value , values[i]);
            assert_eq!(lcdc_register.display_enable, i as u8);
            assert_eq!(lcdc_register.window_display_select, i as u8);
            assert_eq!(lcdc_register.window_enable, i as u8);
            assert_eq!(lcdc_register.tile_data, i as u8);
            assert_eq!(lcdc_register.background_display_select, i as u8);
            assert_eq!(lcdc_register.sprite_size, i as u8);
            assert_eq!(lcdc_register.sprite_enable, i as u8);
            assert_eq!(lcdc_register.background_enable, i as u8);
        }
    }
}