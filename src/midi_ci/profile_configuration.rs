trait MidiProfileInquiryMessages {
    // 7.2 Profile Inquiry Message
    fn profile_inquiry_message(&self) {}
    
    // 7.3 Reply to Profile Inquiry Message
    fn reply_to_profile_inquiry_message(&self) {}
    
    // 7.4 Set Profile On Message
    fn set_profile_on_message(&self) {}
    
    // 7.5 Set Profile Off Message
    fn set_profile_off_message(&self) {}
    
    // 7.6 Profile Enabled Report Message
    fn profile_enabled_report(&self) {}
    
    // 7.7 Profile Disabled Report Message
    fn profile_disabled_report(&self) {}
    
    // 7.8 Profile Specific Data Message
    fn profile_specific_data_message(&self) {}
} 
