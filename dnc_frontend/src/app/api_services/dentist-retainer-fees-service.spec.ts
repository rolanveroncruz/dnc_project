import { TestBed } from '@angular/core/testing';

import { DentistRetainerFeesService } from './dentist-retainer-fees-service';

describe('DentistRetainerFeesService', () => {
  let service: DentistRetainerFeesService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DentistRetainerFeesService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
