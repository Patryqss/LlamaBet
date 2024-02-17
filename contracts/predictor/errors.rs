use psp22::PSP22Error;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PredictorError {
    CallerIsNotAdmin,
    CallerIsNotMarket,

    MintForNotExistingMarket,
    MintOverflow,
    MintTransferFromError(PSP22Error),
    MintAError(PSP22Error),
    MintBError(PSP22Error),
    
    BurnForNotExistingMarket,
    BurnAError(PSP22Error),
    BurnBError(PSP22Error),
    BurnTransferError(PSP22Error),
    BurnOverflow,
}