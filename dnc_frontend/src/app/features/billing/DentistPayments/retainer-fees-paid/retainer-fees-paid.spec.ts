import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RetainerFeesPaid } from './retainer-fees-paid';

describe('RetainerFeesPaid', () => {
  let component: RetainerFeesPaid;
  let fixture: ComponentFixture<RetainerFeesPaid>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RetainerFeesPaid]
    })
    .compileComponents();

    fixture = TestBed.createComponent(RetainerFeesPaid);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
