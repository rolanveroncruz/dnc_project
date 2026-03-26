import { TestBed } from '@angular/core/testing';

import { MemberServicesCountsService } from './member-services-counts-service';

describe('MemberServicesCountsService', () => {
  let service: MemberServicesCountsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(MemberServicesCountsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
