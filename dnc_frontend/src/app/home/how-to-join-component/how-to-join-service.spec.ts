import { TestBed } from '@angular/core/testing';

import { HowToJoinService } from './how-to-join-service';

describe('HowToJoinService', () => {
  let service: HowToJoinService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(HowToJoinService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
