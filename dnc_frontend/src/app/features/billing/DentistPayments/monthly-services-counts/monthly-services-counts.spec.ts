import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MonthlyServicesCounts } from './monthly-services-counts';

describe('MonthlyServicesCounts', () => {
  let component: MonthlyServicesCounts;
  let fixture: ComponentFixture<MonthlyServicesCounts>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MonthlyServicesCounts]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MonthlyServicesCounts);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
