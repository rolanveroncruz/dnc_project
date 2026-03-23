import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SpecialServicesFeesTabComponent } from './special-services-fees-tab-component';

describe('SpecialServicesFeesTabComponent', () => {
  let component: SpecialServicesFeesTabComponent;
  let fixture: ComponentFixture<SpecialServicesFeesTabComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SpecialServicesFeesTabComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SpecialServicesFeesTabComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
