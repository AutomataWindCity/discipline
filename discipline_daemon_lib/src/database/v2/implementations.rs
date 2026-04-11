use crate::x::*;
use super::*;

// u8
impl ScalarWrite for u8 {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_u8(*self);
  }
}

impl ScalarIndexedRead for u8 {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    source.read_u8(index)
  }
}

// u16
impl ScalarWrite for u16 {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_u16(*self);
  }
}

impl ScalarIndexedRead for u16 {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    source.read_u16(index)
  }
}

// u32
impl ScalarWrite for u32 {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_u32(*self);
  }
}

impl ScalarIndexedRead for u32 {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    source.read_u32(index)
  }
}

// u64
impl ScalarWrite for u64 {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_u64(*self);
  }
}

impl ScalarIndexedRead for u64 {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    source.read_u64(index)
  }
}

// i8
impl ScalarWrite for i8 {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_i8(*self);
  }
}

impl ScalarIndexedRead for i8 {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    source.read_i8(index)
  }
}

// i16
impl ScalarWrite for i16 {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_i16(*self);
  }
}

impl ScalarIndexedRead for i16 {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    source.read_i16(index)
  }
}

// i32
impl ScalarWrite for i32 {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_i32(*self);
  }
}

impl ScalarIndexedRead for i32 {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    source.read_i32(index)
  }
}

// i64
impl ScalarWrite for i64 {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_i64(*self);
  }
}

impl ScalarIndexedRead for i64 {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    source.read_i64(index)
  }
}

// str
impl<'a> ScalarWrite for &'a str {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_string(*self);
  }
}

impl ScalarIndexedRead for String {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    source.read_string(index)
  }
}

// Time
impl ScalarWrite for Time {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    self.as_timestamp().write(destination);
  }
}

impl ScalarIndexedRead for Time {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    Time::from_timestamp(source.read_u32(index)?).map_err(|_| ())
  }
}

// Duration
impl ScalarWrite for Duration {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    self.as_total_milliseconds().write(destination);
  }
}

impl ScalarIndexedRead for Duration {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    Ok(Duration::from_milliseconds(source.read_u64(index)?))
  }
}

// Instant
impl ScalarWrite for Instant {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    self.as_elapsed_time().write(destination);
  }
}

impl ScalarIndexedRead for Instant {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    source.read_scalar(index).map(Instant::from_elapsed_time)
  }
}

// Countdown
impl OrderedWriteNull for Countdown {
  fn ordered_write_null(destination: &mut impl OrderedWriteNullDestination) {
    // from
    destination.write_null();
    // duration
    destination.write_null();
  }
}

impl OrderedWrite for Countdown {
  fn ordered_write(&self, destination: &mut impl OrderedWriteDestination) {
    self.get_from().ordered_write(destination);
    self.get_total_duration().ordered_write(destination);
  }
}

pub struct CountdownNames {
  pub from: Name,
  pub duration: Name,
}

impl NamedWrite for Countdown {
  type Names = CountdownNames;

  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination) {
    destination.write_scalar(names.from, &self.get_from());
    destination.write_scalar(names.duration, &self.get_total_duration());
  }
}

pub struct CountdownIndexes {
  pub from: Index,
  pub duration: Index,
}

impl CompoundIndexedRead for Countdown {
  type Indexes = CountdownIndexes;

  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()> {
    Ok(Countdown::construct(
      source.read_scalar(indexes.from)?, 
      source.read_scalar(indexes.duration)?,
    ))
  }
}

// TimeRange
impl OrderedWrite for TimeRange {
  fn ordered_write(&self, destination: &mut impl OrderedWriteDestination) {
    self.from().ordered_write(destination);
    self.till().ordered_write(destination);
  }
}

pub struct TimeRangeNames {
  pub from: Name,
  pub till: Name,
}

impl NamedWrite for TimeRange {
  type Names = TimeRangeNames;

  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination) {
    destination.write_scalar(names.from, self.get_from());
    destination.write_scalar(names.till, self.get_till());
  }
}

pub struct TimeRangeIndexes {
  pub from: Index,
  pub till: Index,
}

impl CompoundIndexedRead for TimeRange {
  type Indexes = TimeRangeIndexes;

  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()> {
    Ok(TimeRange::from_times(
      source.read_scalar(indexes.from)?, 
      source.read_scalar(indexes.till)?,
    ))
  }
}

// MonotonicClock
pub struct MonotonicClockNames {
  pub total_elapsed_duration: Name,
  pub previous_synchronization_boottime: Name,
  pub previous_synchronization_realtime: Name,
  pub maximum_synchronization_interval: Name,
}

impl NamedWrite for MonotonicClock {
  type Names = MonotonicClockNames;
  
  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination) {
    destination.write_scalar(names.total_elapsed_duration, &self.total_elapsed_duration);
    destination.write_scalar(names.previous_synchronization_boottime, &self.previous_synchronization_boottime);
    destination.write_scalar(names.previous_synchronization_realtime, &self.previous_synchronization_realtime);
    destination.write_scalar(names.maximum_synchronization_interval, &self.maximum_synchronization_interval);
  }
}

impl OrderedWrite for MonotonicClock {
  fn ordered_write(&self, destination: &mut impl OrderedWriteDestination) {
    self.total_elapsed_duration.write(destination);
    self.previous_synchronization_boottime.write(destination);
    self.previous_synchronization_realtime.write(destination);
    self.maximum_synchronization_interval.write(destination);
  }
}

pub struct MonotonicClockIndexes {
  pub total_elapsed_duration: Index,
  pub previous_synchronization_boottime: Index,
  pub previous_synchronization_realtime: Index,
  pub maximum_synchronization_interval: Index,
}

impl CompoundIndexedRead for MonotonicClock {
  type Indexes = MonotonicClockIndexes;
  
  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()> {
    Ok(MonotonicClock {
      total_elapsed_duration: source.read_scalar(indexes.total_elapsed_duration)?,
      previous_synchronization_boottime: source.read_scalar(indexes.previous_synchronization_boottime)?,
      previous_synchronization_realtime: source.read_scalar(indexes.previous_synchronization_realtime)?,
      maximum_synchronization_interval: source.read_scalar(indexes.maximum_synchronization_interval)?,
    })
  }
}


// OptionVariant
impl ScalarWrite for OptionVariant {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_u8(self.to_number());
  }
}

impl ScalarIndexedRead for OptionVariant {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    let number = source.read_u8(index)?;
    Self::from_number_or_textual_error(number).ok_or(())
  }
}

// Option
pub struct OptionNames<Value>
where 
  Value: NamedWrite
{
  tag: Name,
  value: Value::Names,
}

impl<Value> NamedWrite for Option<Value> 
where 
  Value: NamedWrite + NamedWriteNull
{
  type Names = OptionNames<Value>;

  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination) {
    match self {
      Self::None => {
        destination.write_scalar(names.tag, &OptionVariant::None);
        Value::named_write_null(&names.value, destination);
      }
      Self::Some(value) => {
        destination.write_scalar(names.tag, &OptionVariant::Some);
        destination.write_compound(&names.value, value);
      }
    }
  }
}

impl<Value> OrderedWrite for Option<Value> 
where 
  Value: OrderedWrite + OrderedWriteNull
{
  fn ordered_write(&self, destination: &mut impl OrderedWriteDestination) {
    match self {
      Self::None => {
        destination.write_scalar(&OptionVariant::None);
        Value::ordered_write_null(destination.as_ordered_write_null_destination());  
      }
      Self::Some(value) => {
        destination.write_scalar(&OptionVariant::None);
        value.ordered_write(destination);
      }
    }
  }
}

pub struct OptionIndexes<Value> 
where
  Value: CompoundIndexedRead
{
  pub tag: Index,
  pub value: Value::Indexes,
}

impl<Value> CompoundIndexedRead for Option<Value> 
where 
  Value: CompoundIndexedRead
{
  type Indexes = OptionIndexes<Value>;

  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()> {
    match source.read_scalar(indexes.tag)? {
      OptionVariant::None => {
        Ok(None)
      }
      OptionVariant::Some => {
        Ok(Some(source.read_compound(&indexes.value)?))
      }
    }
  }
}

// CountdownConditional
pub struct CountdownConditionalNames {
  pub duration: Name,
  pub countdown: OptionNames<CountdownNames>,
}

impl NamedWrite for CountdownConditional {
  type Names = CountdownConditionalNames;
  
  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination) {
    destination.write_scalar(names.duration, &self.duration);
    destination.write_scalar(names.countdown, &self.countdown);
  }
}



impl CompoundIndexedRead for CountdownConditional {
  type Indexes = CountdownConditionalIndexes;
  
  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()> {
    Ok(CountdownConditional {
      duration: source.read_scalar(indexes.duration)?,
      countdown: source.read_scalar(indexes.countdown)?,
    })
  }
}

pub struct CountdownConditionalIndexes {
  pub duration: Index,
  pub countdown: Index,
}

// CountdownAfterPleaConditional
impl NamedWrite for CountdownAfterPleaConditional {
  type Names = CountdownAfterPleaConditionalNames;
  
  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination) {
    destination.write_scalar(names.duration, &self.duration);
    destination.write_scalar(names.countdown, &self.countdown);
  }
}

pub struct CountdownAfterPleaConditionalNames {
  pub duration: Name,
  pub countdown: Name,
}

impl CompoundIndexedRead for CountdownAfterPleaConditional {
  type Indexes = CountdownAfterPleaConditionalIndexes;
  
  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()> {
    Ok(CountdownAfterPleaConditional {
      duration: source.read_scalar(indexes.duration)?,
      countdown: source.read_scalar(indexes.countdown)?,
    })
  }
}

pub struct CountdownAfterPleaConditionalIndexes {
  pub duration: Index,
  pub countdown: Index,
}

// RuleEnablerVariant
impl ScalarWrite for RuleEnablerVariant {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_u8(self.to_number());
  }
}

impl ScalarIndexedRead for RuleEnablerVariant {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    let number = source.read_u8(index)?;
    Self::from_number(number).ok_or(())
  }
}

// RuleEnabler
impl NamedWrite for RuleEnabler {
  type Names = RuleEnablerNames;
  
  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination) {
    match self {
      RuleEnabler::Countdown(conditional) => {
        destination.write_scalar(names.variant, &RuleEnablerVariant::Countdown);
        destination.write_scalar(names.conditional, conditional);
      }
      RuleEnabler::CountdownAfterPlea(conditional) => {
        destination.write_scalar(names.variant, &RuleEnablerVariant::CountdownAfterPlea);
        destination.write_scalar(names.conditional, conditional);
      }
    }
  }
}

pub struct RuleEnablerNames {
  pub variant: Name,
  pub conditional: Name,
}

impl CompoundIndexedRead for RuleEnabler {
  type Indexes = RuleEnablerIndexes;
  
  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()> {
    let variant: RuleEnablerVariant = source.read_scalar(indexes.variant)?;
    match variant {
      RuleEnablerVariant::Countdown => {
        Ok(RuleEnabler::Countdown(source.read_scalar(indexes.conditional)?))
      }
      RuleEnablerVariant::CountdownAfterPlea => {
        Ok(RuleEnabler::CountdownAfterPlea(source.read_scalar(indexes.conditional)?))
      }
    }
  }
}

pub struct RuleEnablerIndexes {
  pub variant: Index,
  pub conditional: Index,
}

// AlwaysRule
impl NamedWrite for AlwaysRule {
  type Names = AlwaysRuleNames;
  
  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination) {
    destination.write_scalar(names.enabler, &self.enabler);
  }
}

pub struct AlwaysRuleNames {
  pub enabler: Name,
}

impl CompoundIndexedRead for AlwaysRule {
  type Indexes = AlwaysRuleIndexes;
  
  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()> {
    Ok(AlwaysRule {
      enabler: source.read_scalar(indexes.enabler)?,
    })
  }
}

pub struct AlwaysRuleIndexes {
  pub enabler: Index,
}

// TimeRangeRule
impl NamedWrite for TimeRangeRule {
  type Names = TimeRangeRuleNames;
  
  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination) {
    destination.write_scalar(names.enabler, &self.enabler);
    destination.write_scalar(names.condition, &self.condition);
  }
}

pub struct TimeRangeRuleNames {
  pub enabler: Name,
  pub condition: Name,
}

impl CompoundIndexedRead for TimeRangeRule {
  type Indexes = TimeRangeRuleIndexes;
  
  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()> {
    Ok(TimeRangeRule {
      enabler: source.read_scalar(indexes.enabler)?,
      condition: source.read_scalar(indexes.condition)?,
    })
  }
}

pub struct TimeRangeRuleIndexes {
  pub enabler: Index,
  pub condition: Index,
}

// TimeAllowanceRule
impl NamedWrite for TimeAllowanceRule {
  type Names = TimeAllowanceRuleNames;
  
  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination) {
    destination.write_scalar(names.enabler, &self.enabler);
    destination.write_scalar(names.allowance, &self.allowance);
  }
}

pub struct TimeAllowanceRuleNames {
  pub enabler: Name,
  pub allowance: Name,
}

impl CompoundIndexedRead for TimeAllowanceRule {
  type Indexes = TimeAllowanceRuleIndexes;
  
  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()> {
    Ok(TimeAllowanceRule {
      enabler: source.read_scalar(indexes.enabler)?,
      allowance: source.read_scalar(indexes.allowance)?,
    })
  }
}

pub struct TimeAllowanceRuleIndexes {
  pub enabler: Index,
  pub allowance: Index,
}

// VaultName - assuming it's a newtype around String or similar
impl ScalarWrite for VaultName {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_string(self.as_ref());
  }
}

impl ScalarIndexedRead for VaultName {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    Ok(VaultName::new(source.read_string(index)?))
  }
}

// VaultDatum - assuming it's a newtype around some serializable type
impl ScalarWrite for VaultDatum {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_bytes(self.as_ref());
  }
}

impl ScalarIndexedRead for VaultDatum {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    Ok(VaultDatum::new(source.read_bytes(index)?))
  }
}

// VaultProtectorVariant
impl ScalarWrite for VaultProtectorVariant {
  fn write(&self, destination: &mut impl ScalarWriteDestination) {
    destination.write_u8(self.to_number());
  }
}

impl ScalarIndexedRead for VaultProtectorVariant {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()> {
    let number = source.read_u8(index)?;
    Self::from_number(number).ok_or(())
  }
}

// VaultProtector
impl NamedWrite for VaultProtector {
  type Names = VaultProtectorNames;
  
  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination) {
    match self {
      Self::CountdownAfterPlea(conditional) => {
        destination.write_scalar(names.variant, &VaultProtectorVariant::CountdownAfterPlea);
        destination.write_scalar(names.conditional, conditional);
      }
    }
  }
}

pub struct VaultProtectorNames {
  pub variant: Name,
  pub conditional: Name,
}

impl CompoundIndexedRead for VaultProtector {
  type Indexes = VaultProtectorIndexes;
  
  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()> {
    let variant: VaultProtectorVariant = source.read_scalar(indexes.variant)?;
    match variant {
      VaultProtectorVariant::CountdownAfterPlea => {
        Ok(VaultProtector::CountdownAfterPlea(source.read_scalar(indexes.conditional)?))
      }
    }
  }
}

pub struct VaultProtectorIndexes {
  pub variant: Index,
  pub conditional: Index,
}

// Vault
impl NamedWrite for Vault {
  type Names = VaultNames;
  
  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination) {
    destination.write_scalar(names.name, &self.name);
    destination.write_scalar(names.protector, &self.protector);
  }
}

pub struct VaultNames {
  pub name: Name,
  pub protector: Name,
}

impl CompoundIndexedRead for Vault {
  type Indexes = VaultIndexes;
  
  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()> {
    Ok(Vault {
      name: source.read_scalar(indexes.name)?,
      protector: source.read_scalar(indexes.protector)?,
    })
  }
}

pub struct VaultIndexes {
  pub name: Index,
  pub protector: Index,
}