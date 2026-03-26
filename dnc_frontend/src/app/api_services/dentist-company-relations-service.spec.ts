import { TestBed } from '@angular/core/testing';

import { DentistCompanyRelationsService } from './dentist-company-relations-service';

describe('DentistCompanyRelationsService', () => {
  let service: DentistCompanyRelationsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DentistCompanyRelationsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
