import { TestBed } from '@angular/core/testing';

import { CSRDentistsService } from './csrdentists-service';

describe('CSRDentistsService', () => {
  let service: CSRDentistsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(CSRDentistsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
