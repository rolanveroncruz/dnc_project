import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DashboardOutliersComponent } from './dashboard-outliers-component';

describe('DashboardOutliersComponent', () => {
  let component: DashboardOutliersComponent;
  let fixture: ComponentFixture<DashboardOutliersComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DashboardOutliersComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DashboardOutliersComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
