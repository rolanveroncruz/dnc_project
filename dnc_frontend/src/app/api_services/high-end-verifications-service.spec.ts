import { TestBed } from '@angular/core/testing';

import { HighEndVerificationsService } from './high-end-verifications-service';

describe('HighEndVerificationsService', () => {
  let service: HighEndVerificationsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(HighEndVerificationsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
