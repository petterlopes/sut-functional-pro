// Common Value Objects
export type EntityId = string;

// Contact Value Objects
export type ContactId = string;

export interface Email {
  value: string;
  isPrimary: boolean;
}

export interface Phone {
  e164: string;
  extension?: string;
  phoneType: PhoneType;
  isPrimary: boolean;
}

export enum PhoneType {
  MOBILE = 'MOBILE',
  WORK = 'WORK',
  HOME = 'HOME',
  OTHER = 'OTHER'
}

export enum ContactType {
  PERSON = 'PERSON',
  ORGANIZATION = 'ORGANIZATION',
  DEPARTMENT = 'DEPARTMENT'
}

export enum ContactStatus {
  ACTIVE = 'ACTIVE',
  INACTIVE = 'INACTIVE',
  PENDING = 'PENDING'
}

// OrgUnit Value Objects
export type OrgUnitId = string;

export interface OrgUnitName {
  value: string;
}

// Department Value Objects
export type DepartmentId = string;

export interface DepartmentName {
  value: string;
}

// User Value Objects
export type UserId = string;

export interface Username {
  value: string;
}

export interface UserEmail {
  value: string;
}

export interface Role {
  value: string;
}

// Audit Value Objects
export type AuditEventId = number;

export interface Action {
  value: string;
}

export interface EntityType {
  value: string;
}

// Source Record Value Objects
export type SourceRecordId = string;

export interface Source {
  value: string;
}

export interface SourceKey {
  value: string;
}

export interface Hash {
  value: string;
}

// Webhook Value Objects
export type WebhookReceiptId = string;

export interface Nonce {
  value: string;
}

// Value Object Validation Functions
export class ValueObjectValidator {
  static validateEmail(email: string): boolean {
    return email.includes('@');
  }

  static validatePhone(phone: string): boolean {
    return phone.length > 0;
  }

  static validateName(name: string): boolean {
    return name.trim().length > 0;
  }

  static validateUsername(username: string): boolean {
    return username.trim().length >= 3;
  }

  static validatePassword(password: string): boolean {
    return password.length >= 8;
  }

  static validateRole(role: string): boolean {
    return role.trim().length > 0;
  }
}

// Value Object Factory Functions
export class ValueObjectFactory {
  static createEmail(value: string, isPrimary: boolean = false): Email {
    if (!ValueObjectValidator.validateEmail(value)) {
      throw new Error('Invalid email format');
    }
    return { value, isPrimary };
  }

  static createPhone(e164: string, phoneType: PhoneType, isPrimary: boolean = false, extension?: string): Phone {
    if (!ValueObjectValidator.validatePhone(e164)) {
      throw new Error('E164 phone number cannot be empty');
    }
    return { e164, extension, phoneType, isPrimary };
  }

  static createOrgUnitName(value: string): OrgUnitName {
    if (!ValueObjectValidator.validateName(value)) {
      throw new Error('OrgUnit name cannot be empty');
    }
    return { value };
  }

  static createDepartmentName(value: string): DepartmentName {
    if (!ValueObjectValidator.validateName(value)) {
      throw new Error('Department name cannot be empty');
    }
    return { value };
  }

  static createUsername(value: string): Username {
    if (!ValueObjectValidator.validateUsername(value)) {
      throw new Error('Username must be at least 3 characters long');
    }
    return { value };
  }

  static createUserEmail(value: string): UserEmail {
    if (!ValueObjectValidator.validateEmail(value)) {
      throw new Error('Invalid email format');
    }
    return { value };
  }

  static createRole(value: string): Role {
    if (!ValueObjectValidator.validateRole(value)) {
      throw new Error('Role cannot be empty');
    }
    return { value };
  }
}
